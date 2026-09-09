//! Compile arbitrary source-derived continuations outside the bundled fixtures.
#[allow(dead_code)]
#[path = "../experiments/access_source.rs"]
mod source;
use std::{fmt::Write, fs, process::Command};
#[test]
fn independent_artifact_preserves_rollback_and_changed_queries() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = root.parent().unwrap().parent().unwrap();
    let scratch = std::env::temp_dir().join(format!("chr-access-artifact-{}", std::process::id()));
    fs::create_dir_all(scratch.join("src")).unwrap();
    let mut manifest = String::from(
        "[package]\nname=\"access-artifact-gate\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[dependencies]\n",
    );
    for (name, path) in [
        ("chr-compiled", "research/chr-compiled"),
        ("chr-syntax", "crates/chr-syntax"),
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
    let mut main = String::from(
        "#![allow(unused_variables,unused_mut,unused_labels,non_camel_case_types,dead_code)]\nuse chr_compiled::{Core,Frame,Selection,Application,Work,Compiled,PreparedRuleset,Policy,Access}; use chr_compiled::native_access::{Continuation,Range};\n",
    );
    writeln!(
        main,
        "#[allow(dead_code)] #[path={:?}] mod oracle;",
        root.join("../chr-direct-conditional/tests/runtime_support/mod.rs")
    )
    .unwrap();
    writeln!(
        main,
        "#[path={:?}] mod source;",
        root.join("experiments/access_source.rs")
    )
    .unwrap();
    for guarded in [false, true] {
        // The emitted selector never receives a query during generation.
        let code = chr_compiled::generate::emit_access(
            if guarded { "guarded" } else { "plain" },
            &source::rules(guarded),
        )
        .unwrap();
        main.push_str(&code);
    }
    main.push_str(&chr_compiled::generate::emit_access("wake", &source::wake_rules()).unwrap());
    main.push_str(r#"
fn main() {
 let mut checked=0;
 for guarded in [false,true] {
  let rules=source::rules(guarded);
  let prepared=PreparedRuleset::new(rules.clone(),Some(if guarded{guarded_code()}else{plain_code()})).unwrap();
  let control=PreparedRuleset::new(rules.clone(),None).unwrap();
  for rotation in 0..7 {for delayed in [false,true] {
   let query=source::query(rotation,guarded,delayed);
   let expected=oracle::run(&rules,&query,100_000);
   for policy in [Policy::Global,Policy::Active] {for access in [Access::Scan,Access::Indexed] {
    let mut a=prepared.start(query.clone(),policy,access).unwrap();
    let mut b=control.start(query.clone(),policy,access).unwrap();
    a.enable_trace(); b.enable_trace();
    assert!(a.advance(100_000).exhausted); assert!(b.advance(100_000).exhausted);
    assert_eq!(a.trace(),b.trace());
    let got=a.observe().into_iter().collect::<Vec<_>>();
    oracle::same_raw(got.clone(),b.observe().into_iter().collect());
    if policy==Policy::Global {oracle::same_raw(got,expected.clone());}
    checked+=1;
   }}
  }}
 }
 let rules=source::wake_rules(); let query=source::wake_query();
 let prepared=PreparedRuleset::new(rules.clone(),Some(wake_code())).unwrap();
 let control=PreparedRuleset::new(rules.clone(),None).unwrap();
 let expected=oracle::run(&rules,&query,100_000);
 assert!(expected[0].residual.iter().any(|c|c.name=="left"));
 for policy in [Policy::Global,Policy::Active] {for access in [Access::Scan,Access::Indexed] {
  let mut a=prepared.start(query.clone(),policy,access).unwrap();
  let mut b=control.start(query.clone(),policy,access).unwrap();
  a.enable_trace();b.enable_trace();
  assert!(a.advance(100_000).exhausted);assert!(b.advance(100_000).exhausted);
  assert_eq!(a.trace(),b.trace(),"wake queue competition {policy:?} {access:?}");
  oracle::same_raw(a.observe().into_iter().collect(),expected.clone());
  checked+=1;
 }}
 assert_eq!(checked,116);
 println!("independent artifact: {checked} complete source/trace checks");
}
"#);
    fs::write(scratch.join("src/main.rs"), main).unwrap();
    let output = Command::new("cargo")
        .args(["run", "--offline", "--quiet", "--manifest-path"])
        .arg(scratch.join("Cargo.toml"))
        .env(
            "CARGO_TARGET_DIR",
            workspace.join("target/access-artifact-gate"),
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("116 complete source/trace checks"));
    print!("{}", String::from_utf8_lossy(&output.stdout));
    fs::remove_dir_all(scratch).unwrap();
}
