//! Emit the existing call sources and attach their shared gate/lifecycle runner.
use std::{fmt::Write, fs, path::Path, time::Instant};
#[allow(dead_code)]
#[path = "support/call_trace_source.rs"]
mod fixture;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 2, "usage: call_generated PROJECT_DIR");
    let out = Path::new(&args[1]);
    assert!(!out.exists(), "preserve existing artifact");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = root.parent().unwrap().parent().unwrap();
    let started = Instant::now();
    let mut generated = String::from(
        "#![allow(unused_variables,unused_mut,unused_labels,non_camel_case_types,dead_code)]\nuse chr_compiled::{Core,Frame,Selection,Application,Work,Compiled};\nuse chr_compiled::native_access::{Continuation,Range};\n",
    );
    for reverse in [false, true] {
        for family in 0..4 {
            generated.push_str(
                &chr_compiled::generate::emit_access(
                    &format!("f{family}_r{}", usize::from(reverse)),
                    &fixture::program(reverse, family).0,
                )
                .unwrap(),
            );
        }
    }
    generated
        .push_str("pub fn code(reverse:bool,family:usize)->Compiled {match(reverse,family){\n");
    for reverse in [false, true] {
        for family in 0..4 {
            writeln!(
                generated,
                "({reverse},{family})=>f{family}_r{}_code(),",
                usize::from(reverse)
            )
            .unwrap();
        }
    }
    generated.push_str("_=>panic!(\"family\")}}\n");
    let emission_ns = started.elapsed().as_nanos();
    let mut manifest = String::from(
        "[package]\nname=\"call-generated-artifact\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[features]\nalloc-meter=[\"chr-reuse/alloc-meter\",\"chr-compiled/alloc-meter\"]\nmetrics=[\"chr-compiled/metrics\"]\n[dependencies]\n",
    );
    for (name, path) in [
        ("chr-reuse", "research/chr-reuse"),
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
    let main = format!(
        "mod generated;\n#[allow(dead_code)] #[path={:?}] mod gate;\n#[allow(dead_code)] #[path={:?}] mod lifecycle;\nfn main(){{let a=std::env::args().collect::<Vec<_>>();if a.get(1).is_some_and(|x|x==\"gate\"){{gate::check(|r,f|Some(generated::code(r,f)));}}else{{let family=a[2].parse().unwrap();lifecycle::entry(Some(generated::code(false,family)));}}}}\n",
        root.join("tests/call_compiled.rs"),
        root.join("examples/call_trace_ownership.rs"),
    );
    fs::create_dir_all(out.join("src")).unwrap();
    fs::write(out.join("Cargo.toml"), manifest).unwrap();
    fs::write(out.join("src/main.rs"), main).unwrap();
    fs::write(out.join("src/generated.rs"), &generated).unwrap();
    println!(
        "{{\"emission_ns\":{emission_ns},\"generated_bytes\":{},\"descriptors\":8}}",
        generated.len()
    );
}
