//! Whole executable-session correspondence; no cost measurements.
#[path = "../experiments/recursive_session_fixture.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_compiled::recursive::{
    Prepared,
    session::{self, Response},
};
use chr_syntax::{Query, Var, atom, c, t, v};
use std::{
    fmt::Write,
    fs,
    io::Write as IoWrite,
    process::{Command, Stdio},
};
fn frame(payload: &[u8], out: &mut Vec<u8>) {
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
}
#[test]
fn three_executables_preserve_complete_changed_query_sessions() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = root.parent().unwrap().parent().unwrap();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let scratch = std::env::temp_dir().join(format!(
        "chr-recursive-session-{}-{stamp}",
        std::process::id()
    ));
    fs::create_dir_all(&scratch).unwrap();
    let mut input = Vec::new();
    frame(&[255], &mut input);
    let mut queries = vec![];
    for n in [0, 1, 4, 16] {
        let control = (0..n).fold(atom("z"), |x, _| t("s", [x]));
        for result in [v(11), v(10), atom("bad")] {
            queries.push(Query {
                constraints: vec![c("add", [control.clone(), v(10), result])],
                outputs: vec![
                    ("x".into(), Var(10)),
                    ("y".into(), Var(11)),
                    ("unused".into(), Var(99)),
                ],
            });
        }
    }
    let unsupported = Query {
        constraints: vec![c("add", [v(12), v(10), v(11)])],
        outputs: vec![("x".into(), Var(10)), ("y".into(), Var(11))],
    };
    queries.push(unsupported);
    for q in &queries {
        frame(&session::encode_query(q).unwrap(), &mut input);
    }
    for backend in ["native", "direct", "specialized"] {
        let dir = scratch.join(backend);
        fs::create_dir_all(dir.join("src")).unwrap();
        let mut manifest = format!(
            "[package]\nname=\"session-{backend}\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[dependencies]\n"
        );
        for (name, path) in [
            ("chr-syntax", "crates/chr-syntax"),
            (
                if backend == "native" {
                    "chr-persistent"
                } else {
                    "chr-compiled"
                },
                if backend == "native" {
                    "research/chr-persistent"
                } else {
                    "research/chr-compiled"
                },
            ),
        ] {
            writeln!(
                manifest,
                "{name}={{path={:?},default-features=false}}",
                workspace.join(path)
            )
            .unwrap();
        }
        fs::write(dir.join("Cargo.toml"), manifest).unwrap();
        let mut main = format!(
            "#[path={:?}] mod session;\nuse session::Response;\n",
            root.join("src/recursive/session.rs")
        );
        if backend == "native" {
            fs::write(
                dir.join("src/generated.rs"),
                Prepared::new(fixture::rules("add")).unwrap().emit_rust(),
            )
            .unwrap();
            main.push_str("mod generated; fn main(){session::serve(std::io::stdin().lock(),std::io::stdout().lock(),|q|match generated::execute(q){Ok(Some(a))=>Response::Success(a),Ok(None)=>Response::Failure,Err(e)=>Response::Unsupported(e)}).unwrap();}");
        } else {
            writeln!(
                main,
                "#[path={:?}] mod fixture;",
                root.join("experiments/recursive_session_fixture.rs")
            )
            .unwrap();
            if backend == "direct" {
                main.push_str("fn main(){let p=chr_compiled::recursive::Prepared::new(fixture::rules(\"add\")).unwrap();session::serve(std::io::stdin().lock(),std::io::stdout().lock(),|q|match p.execute(q){Ok(Some(a))=>Response::Success(a),Ok(None)=>Response::Failure,Err(e)=>Response::Unsupported(e.0)}).unwrap();}");
            } else {
                main.push_str(r#"fn main(){let p=chr_compiled::PreparedRuleset::new(fixture::rules("add"),None).unwrap().specialize_inferred();session::serve(std::io::stdin().lock(),std::io::stdout().lock(),|q|{
let e=match p.start(q,chr_compiled::Policy::Global,chr_compiled::Access::Indexed){Ok(e)=>e,Err(e)=>return Response::Error(e)};
let mut e=e.into_search();let mut answer=None;
for _ in 0..100_000 {match e.tick(){chr_compiled::SearchEvent::Complete(mut b)=>{if answer.is_some(){return Response::Error("unexpected raw multiplicity".into());}match b.engine.observe(){Some(a)=>answer=Some(a),None=>return Response::Error("observation unavailable".into())}},chr_compiled::SearchEvent::Exhausted=>return answer.map_or(Response::Failure,Response::Success),_=>()}}
Response::Error("service cutoff".into())}).unwrap();}"#);
            }
        }
        fs::write(dir.join("src/main.rs"), main).unwrap();
        let build = Command::new(env!("CARGO"))
            .args(["build", "--offline", "--quiet", "--manifest-path"])
            .arg(dir.join("Cargo.toml"))
            .arg("--target-dir")
            .arg(scratch.join("target"))
            .output()
            .unwrap();
        assert!(
            build.status.success(),
            "{backend} build at {}: {}",
            scratch.display(),
            String::from_utf8_lossy(&build.stderr)
        );
        let mut child = Command::new(scratch.join(format!("target/debug/session-{backend}")))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(&input).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{backend}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let mut bytes = output.stdout.as_slice();
        let mut responses = vec![];
        while !bytes.is_empty() {
            assert!(bytes.len() >= 4);
            let n = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
            bytes = &bytes[4..];
            assert!(bytes.len() >= n);
            responses.push(session::decode_response(&bytes[..n]).unwrap());
            bytes = &bytes[n..];
        }
        assert_eq!(responses.len(), queries.len() + 1);
        assert!(matches!(responses[0], Response::Malformed(_)));
        for (i, (q, response)) in queries
            .iter()
            .zip(responses.into_iter().skip(1))
            .enumerate()
        {
            if i == queries.len() - 1 && backend != "specialized" {
                assert!(matches!(response, Response::Unsupported(_)));
                continue;
            }
            let expected = scalar::run(&fixture::rules("add"), q, 100_000);
            let actual = match response {
                Response::Success(a) => vec![a],
                Response::Failure => vec![],
                other => panic!("unexpected {backend} response {other:?}"),
            };
            scalar::same_raw(actual, expected);
        }
    }
    println!(
        "validated 42 session responses across three executables; artifact at {}",
        scratch.display()
    );
}
