use super::{
    Backend, Compiled, Local, Phase, WORK, phase, scalar, selection, selection_source as src,
};
use chr_syntax::Answer;
fn validate(answer: &Option<Answer>, expected: &[Answer]) {
    match (answer, expected) {
        (None, []) => (),
        (Some(a), [b]) => assert!(chr_observe::equivalent(a, b, &mut Default::default())),
        _ => panic!("answer cardinality"),
    }
}
fn measure<B: Backend, const CHR: bool>(family: &str, n: usize, retention: &str, cancel: bool) {
    let expected =
        ["a", "b"].map(|v| scalar::run(&[src::rule()], &src::query(family, n, v), 2_000_000));
    let mut phases: Vec<(&str, usize, Phase)> = Vec::with_capacity(40);
    let mut held = Vec::with_capacity(4);
    let mut endpoints = Vec::with_capacity(4);
    let mut work = Vec::with_capacity(if WORK { 4 } else { 0 });
    println!("{{\"event\":\"start\"}}");
    #[cfg(feature = "alloc-meter")]
    let baseline = super::meter::begin();
    let (rules, p) = phase(|| {
        if CHR {
            selection::rules(true)
        } else {
            vec![src::rule()]
        }
    });
    phases.push(("source", 0, p));
    let (prepared, p) = phase(|| B::prepare_rules(&rules));
    phases.push(("prepare", 0, p));
    for i in 0..4 {
        let stopped = cancel && i % 2 == 0;
        let (q, p) = phase(|| src::query(family, n, if i % 2 == 0 { "a" } else { "b" }));
        phases.push(("input", i, p));
        let (encoded, p) = phase(|| if CHR { Some(src::encode(&q)) } else { None });
        phases.push(("encode", i, p));
        let (mut state, p) = phase(|| prepared.setup(encoded.as_ref().map_or(&q, |e| &e.query)));
        phases.push(("setup", i, p));
        let (complete, p) = phase(|| B::advance(&mut state, if stopped { 1 } else { 2_000_000 }));
        phases.push(("execute", i, p));
        assert!(stopped || complete, "service cutoff");
        let (answer, p) = phase(|| {
            if stopped {
                None
            } else {
                B::observe(&mut state).map(|a| {
                    if let Some(e) = &encoded {
                        src::decode(a, e)
                    } else {
                        a
                    }
                })
            }
        });
        phases.push(("observe", i, p));
        endpoints.push((stopped, complete, answer.is_some()));
        if WORK {
            work.push(B::work(&state));
        }
        let (_, p) = phase(|| drop(state));
        phases.push(("engine_drop", i, p));
        let (_, p) = phase(|| drop((encoded, q)));
        phases.push(("input_drop", i, p));
        if !stopped {
            validate(&answer, &expected[i % 2]);
        }
        let (_, p) = phase(|| {
            if retention == "all" {
                held.push((i, answer));
            } else {
                drop(answer);
            }
        });
        phases.push(("consumer", i, p));
    }
    let (_, p) = phase(|| drop(prepared));
    phases.push(("prepared_drop", 0, p));
    let (_, p) = phase(|| drop(rules));
    phases.push(("source_drop", 0, p));
    for (i, a) in &held {
        if !(cancel && i % 2 == 0) {
            validate(a, &expected[i % 2]);
        }
    }
    let (_, p) = phase(|| held.clear());
    phases.push(("consumer_drop", 0, p));
    #[cfg(feature = "alloc-meter")]
    {
        let end = super::meter::end(baseline);
        assert_eq!(end.live_start, end.live_end, "unreleased ownership");
    }
    let rows = phases
        .into_iter()
        .map(|(name, q, p)| {
            format!(
                "{{\"phase\":\"{name}\",\"query\":{q},\"reading\":{}}}",
                p.json()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let ends = endpoints
        .iter()
        .map(|(c, e, o)| format!("{{\"cancelled\":{c},\"exhausted\":{e},\"observed\":{o}}}"))
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"event\":\"result\",\"chr\":{CHR},\"work_enabled\":{WORK},\"cancel\":{cancel},\"meter\":{},\"endpoints\":[{ends}],\"phases\":[{rows}],\"work\":[{}]}}",
        cfg!(feature = "alloc-meter"),
        work.join(",")
    );
}
pub fn main(args: &[String]) {
    #[cfg(feature = "alloc-meter")]
    if args == ["meter-check"] {
        super::meter::self_check().unwrap();
        println!("meter-check passed");
        return;
    }
    assert_eq!(
        args.len(),
        5,
        "selection mode family size immediate|all complete|cancel"
    );
    if cfg!(feature = "alloc-meter") && WORK {
        panic!("allocation and work builds must be separate");
    }
    assert_eq!(WORK, chr_compiled::COLLECT_METRICS);
    let family = &args[1];
    let n = args[2].parse().unwrap();
    let retention = &args[3];
    assert!(["immediate", "all"].contains(&retention.as_str()));
    let cancel = match args[4].as_str() {
        "complete" => false,
        "cancel" => true,
        _ => panic!("invalid stop"),
    };
    match args[0].as_str() {
        "local-filtered" => measure::<Local<1>, false>(family, n, retention, cancel),
        "local" => measure::<Local<0>, false>(family, n, retention, cancel),
        "scan" => measure::<Compiled<false, false>, false>(family, n, retention, cancel),
        "indexed" => measure::<Compiled<true, false>, false>(family, n, retention, cancel),
        "sealed" => measure::<Compiled<false, true>, false>(family, n, retention, cancel),
        "chr-scan" => measure::<Compiled<false, false>, true>(family, n, retention, cancel),
        "chr-indexed" => measure::<Compiled<true, false>, true>(family, n, retention, cancel),
        "chr-sealed" => measure::<Compiled<false, true>, true>(family, n, retention, cancel),
        _ => panic!("invalid control"),
    }
}
