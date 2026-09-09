//! Compile emitted modules outside chr-compiled and run independent full-answer checks.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_compiled::recursive::Prepared;
use chr_syntax::{Answer, Query, Rule, Term, Var, and, atom, c, eq, t, v};
use std::{fmt::Write, fs, process::Command};
fn term(x: &Term) -> String {
    match x {
        Term::Var(Var(id)) => format!("v({id})"),
        Term::App(n, xs) => format!(
            "t({n:?},vec![{}])",
            xs.iter().map(term).collect::<Vec<_>>().join(",")
        ),
    }
}
fn query(q: &Query) -> String {
    format!(
        "Query{{constraints:vec![{}],outputs:vec![{}]}}",
        q.constraints
            .iter()
            .map(|x| format!(
                "c({:?},vec![{}])",
                x.name,
                x.args.iter().map(term).collect::<Vec<_>>().join(",")
            ))
            .collect::<Vec<_>>()
            .join(","),
        q.outputs
            .iter()
            .map(|(n, Var(id))| format!("({n:?}.into(),Var({id}))"))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn answer(a: &Answer) -> String {
    assert!(a.residual.is_empty());
    format!(
        "Answer{{outputs:vec![{}],residual:vec![]}}",
        a.outputs
            .iter()
            .map(|(n, x)| format!("({n:?}.into(),{})", term(x)))
            .collect::<Vec<_>>()
            .join(",")
    )
}
#[test]
fn emitted_artifact_preserves_changed_queries_and_admission() {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let scratch = std::env::temp_dir().join(format!(
        "chr-recursive-native-gate-{}-{stamp}",
        std::process::id()
    ));
    fs::create_dir_all(scratch.join("src")).unwrap();
    let mut manifest = String::from(
        "[package]\nname=\"recursive-native-gate\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[dependencies]\n",
    );
    for (name, path) in [
        ("chr-syntax", "crates/chr-syntax"),
        ("chr-persistent", "research/chr-persistent"),
        ("chr-observe", "research/chr-observe"),
    ] {
        writeln!(
            manifest,
            "{name}={{path={:?},default-features=false}}",
            workspace.join(path)
        )
        .unwrap();
    }
    fs::write(scratch.join("Cargo.toml"), manifest).unwrap();
    let mut main = String::from("use chr_syntax::{Answer,Query,Var,c,t,v};\n");
    let mut checks = String::from("fn main(){\nlet mut checked=0;\n");
    let mut module = 0;
    let mut expected_count = 0;
    for accumulate in [false, true] {
        for position in 0..3 {
            let pred = "relation\"\\\nλ";
            let base = "done\"\\\n终";
            let step = "more\"\\\n枝";
            let permute = |mut xs: Vec<Term>| {
                xs.swap(0, position);
                xs
            };
            let body = if accumulate {
                and(vec![
                    eq(v(3), t("pair", [v(1), v(4)])),
                    c(pred, permute(vec![v(0), v(3), v(2)])).into(),
                ])
            } else {
                and(vec![
                    eq(v(2), t("wrap", [v(3)])),
                    c(pred, permute(vec![v(0), v(1), v(3)])).into(),
                ])
            };
            let rules = vec![
                Rule::simplify(
                    "base",
                    [c(pred, permute(vec![atom(base), v(1), v(2)]))],
                    eq(v(1), v(2)),
                ),
                Rule::simplify(
                    "step",
                    [c(pred, permute(vec![t(step, [v(0)]), v(1), v(2)]))],
                    body,
                ),
            ];
            let p = Prepared::new(rules.clone()).unwrap();
            fs::write(scratch.join(format!("src/m{module}.rs")), p.emit_rust()).unwrap();
            writeln!(main, "mod m{module};").unwrap();
            let make = |control, payload, result| Query {
                constraints: vec![c(pred, permute(vec![control, payload, result]))],
                outputs: vec![
                    ("x\"\\".into(), Var(10)),
                    ("y".into(), Var(11)),
                    ("unused".into(), Var(99)),
                ],
            };
            for depth in [0, 3] {
                let control = (0..depth).fold(atom(base), |x, _| t(step, [x]));
                for payload in [v(10), atom("a"), t("pair", [v(10), v(11)])] {
                    for result in [v(11), v(10), atom("b"), t("wrap", [v(10)])] {
                        let q = make(control.clone(), payload.clone(), result);
                        let expected = scalar::run(&rules, &q, 100_000);
                        assert!(expected.len() <= 1);
                        scalar::same_raw(
                            p.execute(q.clone()).unwrap().into_iter().collect(),
                            expected.clone(),
                        );
                        let value = expected
                            .first()
                            .map_or_else(|| "None".into(), |a| format!("Some({})", answer(a)));
                        writeln!(checks,"{{let actual=m{module}::execute({}).unwrap();let expected:Option<Answer>={value};match(actual,expected){{(Some(a),Some(b))=>assert!(chr_observe::equivalent(&a,&b,&mut Default::default())),(None,None)=>(),x=>panic!(\"outcome mismatch: {{x:?}}\")}}checked+=1;}}",query(&q)).unwrap();
                        expected_count += 1;
                    }
                }
            }
            for control in [
                v(77),
                t(step, [v(77)]),
                atom("foreign"),
                t(base, [atom(base)]),
            ] {
                let q = make(control, v(10), v(11));
                writeln!(
                    checks,
                    "assert!(m{module}::execute({}).is_err());checked+=1;",
                    query(&q)
                )
                .unwrap();
                expected_count += 1;
            }
            let mut q = make(atom(base), v(10), v(11));
            q.constraints.push(c("outside", []));
            writeln!(
                checks,
                "assert!(m{module}::execute({}).is_err());checked+=1;",
                query(&q)
            )
            .unwrap();
            expected_count += 1;
            let mut q = make(atom(base), v(10), v(11));
            q.outputs.push(q.outputs[0].clone());
            writeln!(
                checks,
                "assert!(m{module}::execute({}).is_err());checked+=1;",
                query(&q)
            )
            .unwrap();
            expected_count += 1;
            module += 1;
        }
    }
    let rules = vec![
        Rule::simplify("done", [c("unit", [atom("node")])], chr_syntax::Goal::True),
        Rule::simplify(
            "recur",
            [c("unit", [t("node", [v(0)])])],
            c("unit", [v(0)]).into(),
        ),
    ];
    let p = Prepared::new(rules.clone()).unwrap();
    fs::write(scratch.join(format!("src/m{module}.rs")), p.emit_rust()).unwrap();
    writeln!(main, "mod m{module};").unwrap();
    for depth in [0, 3] {
        let q = Query {
            constraints: vec![c(
                "unit",
                [(0..depth).fold(atom("node"), |x, _| t("node", [x]))],
            )],
            outputs: vec![("unused".into(), Var(99))],
        };
        let expected = scalar::run(&rules, &q, 100_000);
        assert_eq!(expected.len(), 1);
        writeln!(checks,"{{let actual=m{module}::execute({}).unwrap().unwrap();let expected={};assert!(chr_observe::equivalent(&actual,&expected,&mut Default::default()));checked+=1;}}",query(&q),answer(&expected[0])).unwrap();
        expected_count += 1;
    }
    module += 1;
    writeln!(
        checks,
        "println!(\"validated {{checked}} generated outcomes\");}}"
    )
    .unwrap();
    main.push_str(&checks);
    fs::write(scratch.join("src/main.rs"), main).unwrap();
    let output = Command::new(env!("CARGO"))
        .args(["run", "--offline", "--quiet", "--manifest-path"])
        .arg(scratch.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(scratch.join("target"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "artifact gate failed at {}:\n{}\n{}",
        scratch.display(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        format!("validated {expected_count} generated outcomes")
    );
    println!(
        "validated {expected_count} outcomes across {module} modules; artifact at {}",
        scratch.display()
    );
}
