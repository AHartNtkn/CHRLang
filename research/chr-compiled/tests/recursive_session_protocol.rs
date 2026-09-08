use chr_compiled::recursive::session::{self, Response};
use chr_syntax::{Answer, Query, Var, atom, c, t, v};
use std::io::Cursor;
fn input() -> Query {
    Query {
        constraints: vec![c("p", [v(4), t("f", [v(4), atom("λ")])])],
        outputs: vec![("x".into(), Var(4)), ("unused".into(), Var(9))],
    }
}
fn frame(bytes: &[u8]) -> Vec<u8> {
    let mut out = (bytes.len() as u32).to_le_bytes().to_vec();
    out.extend(bytes);
    out
}
#[test]
fn exact_handcoded_query_and_joint_answer_roundtrip() {
    // One p(V7) constraint, one output named x selecting V7.
    let bytes = vec![
        1, 0, 0, 0, 1, 0, 0, 0, b'p', 1, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0,
        0, b'x', 7, 0, 0, 0, 0, 0, 0, 0,
    ];
    let q = Query {
        constraints: vec![c("p", [v(7)])],
        outputs: vec![("x".into(), Var(7))],
    };
    assert_eq!(session::encode_query(&q).unwrap(), bytes);
    assert_eq!(session::decode_query(&bytes).unwrap(), q);
    let q = input();
    assert_eq!(
        session::decode_query(&session::encode_query(&q).unwrap()).unwrap(),
        q
    );
    let response = Response::Success(Answer {
        outputs: vec![("x".into(), v(42)), ("y".into(), t("f", [v(42)]))],
        residual: vec![c("r", [v(42), v(43)]), c("r", [v(42), v(43)])],
    });
    assert_eq!(
        session::decode_response(&session::encode_response(&response).unwrap()).unwrap(),
        response
    );
    for r in [
        Response::Failure,
        Response::Unsupported("unknown".into()),
        Response::Malformed("bad".into()),
        Response::Error("limit".into()),
    ] {
        assert_eq!(
            session::decode_response(&session::encode_response(&r).unwrap()).unwrap(),
            r
        );
    }
}
#[test]
fn malformed_complete_frame_does_not_poison_changed_requests() {
    let q = input();
    let mut stream = frame(&[255]);
    stream.extend(frame(&session::encode_query(&q).unwrap()));
    let mut changed = q.clone();
    changed.outputs.clear();
    stream.extend(frame(&session::encode_query(&changed).unwrap()));
    let mut output = vec![];
    let mut seen = vec![];
    session::serve(Cursor::new(stream), &mut output, |q| {
        seen.push(q);
        Response::Failure
    })
    .unwrap();
    assert_eq!(seen, vec![q, changed]);
    let mut cursor = 0;
    let mut decoded = vec![];
    while cursor < output.len() {
        let len = u32::from_le_bytes(output[cursor..cursor + 4].try_into().unwrap()) as usize;
        cursor += 4;
        decoded.push(session::decode_response(&output[cursor..cursor + len]).unwrap());
        cursor += len;
    }
    assert!(matches!(decoded[0], Response::Malformed(_)));
    assert_eq!(&decoded[1..], &[Response::Failure, Response::Failure]);
}
#[test]
fn framing_and_limits_are_explicit_errors() {
    for bytes in [vec![1], vec![4, 0, 0, 0, 1, 2]] {
        assert_eq!(
            session::serve(Cursor::new(bytes), vec![], |_| panic!("not decoded"))
                .unwrap_err()
                .kind(),
            std::io::ErrorKind::UnexpectedEof
        );
    }
    assert!(
        session::serve(
            Cursor::new(((session::MAX_FRAME + 1) as u32).to_le_bytes()),
            vec![],
            |_| panic!("oversize")
        )
        .is_err()
    );
    let mut bytes = session::encode_query(&input()).unwrap();
    bytes.push(0);
    assert!(session::decode_query(&bytes).is_err());
    let deep = (0..session::MAX_DEPTH + 1).fold(atom("z"), |x, _| t("s", [x]));
    assert!(
        session::encode_query(&Query {
            constraints: vec![c("p", [deep])],
            outputs: vec![]
        })
        .is_err()
    );
    assert!(session::encode_response(&Response::Error("x".repeat(session::MAX_FRAME))).is_err());
}
#[test]
fn aggregate_nodes_depth_utf8_and_response_resource_failure() {
    // Empty constraints use eight bytes each, so this exceeds nodes before bytes.
    let q = Query {
        constraints: vec![c("", []); session::MAX_NODES + 1],
        outputs: vec![],
    };
    assert!(session::encode_query(&q).unwrap_err().contains("node"));
    let mut bytes = ((session::MAX_NODES + 1) as u32).to_le_bytes().to_vec();
    bytes.extend(vec![0; 8 * (session::MAX_NODES + 1) + 4]);
    assert!(session::decode_query(&bytes).unwrap_err().contains("node"));
    let q = Query {
        constraints: vec![c("", []); session::MAX_NODES],
        outputs: vec![],
    };
    assert_eq!(
        session::decode_query(&session::encode_query(&q).unwrap()).unwrap(),
        q
    );
    // Invalid UTF-8 in a one-byte constraint name.
    let mut bytes = vec![1, 0, 0, 0, 1, 0, 0, 0, 255];
    bytes.extend([0; 8]);
    assert!(session::decode_query(&bytes).unwrap_err().contains("UTF-8"));
    let deepest = (1..session::MAX_DEPTH).fold(atom("z"), |x, _| t("s", [x]));
    let deepest_query = Query {
        constraints: vec![c("p", [deepest])],
        outputs: vec![],
    };
    let deepest_bytes = session::encode_query(&deepest_query).unwrap();
    assert_eq!(
        session::decode_query(&deepest_bytes).unwrap(),
        deepest_query
    );
    // Independently build a too-deep constructor chain in the payload.
    let mut bytes = vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
    for _ in 0..session::MAX_DEPTH {
        bytes.extend([1, 0, 0, 0, 0, 1, 0, 0, 0]);
    }
    bytes.push(0);
    bytes.extend([0; 12]);
    assert!(session::decode_query(&bytes).unwrap_err().contains("depth"));
    let q = input();
    let payload = frame(&session::encode_query(&q).unwrap());
    let mut stream = payload.clone();
    stream.extend(payload);
    let mut output = vec![];
    let mut calls = 0;
    session::serve(Cursor::new(stream), &mut output, |_| {
        calls += 1;
        if calls == 1 {
            Response::Error("x".repeat(session::MAX_FRAME))
        } else {
            Response::Failure
        }
    })
    .unwrap();
    let n = u32::from_le_bytes(output[..4].try_into().unwrap()) as usize;
    assert!(
        matches!(session::decode_response(&output[4..4+n]).unwrap(),Response::Error(message) if message.contains("response encoding"))
    );
    assert_eq!(
        session::decode_response(&output[n + 8..]).unwrap(),
        Response::Failure
    );
    assert_eq!(calls, 2);
}
#[test]
fn each_response_is_flushed_and_clean_eof_never_executes() {
    #[derive(Default)]
    struct Sink {
        bytes: Vec<u8>,
        flushes: usize,
    }
    impl std::io::Write for Sink {
        fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
            self.bytes.extend(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            self.flushes += 1;
            Ok(())
        }
    }
    let mut sink = Sink::default();
    session::serve(Cursor::new([]), &mut sink, |_| panic!("empty")).unwrap();
    assert_eq!(sink.flushes, 0);
    let bytes = frame(&session::encode_query(&input()).unwrap());
    let mut requests = bytes.clone();
    requests.extend(bytes);
    session::serve(Cursor::new(requests), &mut sink, |_| Response::Failure).unwrap();
    assert_eq!(sink.flushes, 2);
    assert_eq!(sink.bytes, vec![1, 0, 0, 0, 1, 1, 0, 0, 0, 1]);
}
