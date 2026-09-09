//! Emit independent session packages; compilation/execution is caller-owned.
#[path = "../experiments/recursive_session_fixture.rs"]
mod fixture;
use chr_compiled::recursive::Prepared;
use std::{fmt::Write, fs, path::Path, time::Instant};
fn package(
    out: &Path,
    workspace: &Path,
    backend: &str,
    native: &str,
    prereq: bool,
) -> std::io::Result<()> {
    let dir = out.join(format!("{backend}{}", if prereq { "-prereq" } else { "" }));
    fs::create_dir_all(dir.join("src"))?;
    let name = if prereq { "prereq" } else { "session" };
    let mut manifest = format!(
        "[package]\nname={name:?}\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[profile.release]\nopt-level=3\ncodegen-units=1\nincremental=false\n[dependencies]\n"
    );
    for (name, path) in [
        ("chr-syntax", "crates/chr-syntax"),
        if backend == "native" {
            ("chr-persistent", "research/chr-persistent")
        } else {
            ("chr-compiled", "research/chr-compiled")
        },
    ] {
        writeln!(
            manifest,
            "{name}={{path={:?},default-features=false}}",
            workspace.join(path)
        )
        .unwrap();
    }
    fs::write(dir.join("Cargo.toml"), manifest)?;
    if prereq {
        return fs::write(dir.join("src/main.rs"), "fn main() {}\n");
    }
    let mut main = format!(
        "#[path={:?}] mod session;\nuse session::Response;\n",
        workspace.join("research/chr-compiled/src/recursive/session.rs")
    );
    if backend == "native" {
        fs::write(dir.join("src/generated.rs"), native)?;
        main.push_str("mod generated;\nfn main() -> std::io::Result<()> {\nlet result=session::serve(std::io::stdin().lock(),std::io::stdout().lock(),|q|match generated::execute(q){Ok(Some(a))=>Response::Success(a),Ok(None)=>Response::Failure,Err(e)=>Response::Unsupported(e)});\neprintln!(\"{{\\\"preparation_ns\\\":0}}\");result}\n");
    } else {
        writeln!(
            main,
            "#[path={:?}] mod fixture;",
            workspace.join("research/chr-compiled/experiments/recursive_session_fixture.rs")
        )
        .unwrap();
        main.push_str("fn main()->std::io::Result<()> {let kind=std::env::args().nth(1).filter(|s|matches!(s.as_str(),\"add\"|\"fresh\")).ok_or_else(||std::io::Error::other(\"requires add|fresh argument\"))?;let started=std::time::Instant::now();\n");
        if backend == "direct" {
            writeln!(main,"let p=chr_compiled::recursive::Prepared::new(fixture::rules(&kind)).expect(\"certified fixture\");").unwrap();
        } else {
            writeln!(main,"let p=chr_compiled::PreparedRuleset::new(fixture::rules(&kind),None).expect(\"source fixture\").specialize_inferred();").unwrap();
        }
        main.push_str("let preparation_ns=started.elapsed().as_nanos();\nlet result=session::serve(std::io::stdin().lock(),std::io::stdout().lock(),|q|{\n");
        if backend == "direct" {
            main.push_str("match p.execute(q){Ok(Some(a))=>Response::Success(a),Ok(None)=>Response::Failure,Err(e)=>Response::Unsupported(e.0)}\n");
        } else {
            main.push_str(r#"let e=match p.start(q,chr_compiled::Policy::Global,chr_compiled::Access::Indexed){Ok(e)=>e,Err(e)=>return Response::Error(e)};
let mut e=e.into_search();let mut answer=None;
for _ in 0..20_000_000 {match e.tick(){
chr_compiled::SearchEvent::Complete(mut b)=>{if answer.is_some(){return Response::Error("unexpected raw multiplicity".into())}answer=match b.engine.observe(){Some(a)=>Some(a),None=>return Response::Error("observation unavailable".into())};},
chr_compiled::SearchEvent::Exhausted=>return answer.map_or(Response::Failure,Response::Success),_=>()}}
Response::Error("service cutoff at 20000000 ticks".into())
"#);
        }
        main.push_str(
            "});drop(p);eprintln!(\"{{\\\"preparation_ns\\\":{preparation_ns}}}\");result}\n",
        );
    }
    fs::write(dir.join("src/main.rs"), main)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total = Instant::now();
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 || !matches!(args[1].as_str(), "add" | "fresh") {
        return Err("usage: recursive_session_packages OUTPUT_DIR add|fresh".into());
    }
    let out = Path::new(&args[0]);
    if out.exists() {
        return Err("output directory already exists".into());
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let start = Instant::now();
    let rules = fixture::rules(&args[1]);
    let fixture_ns = start.elapsed().as_nanos();
    let start = Instant::now();
    let certificate = Prepared::new(rules)?;
    let certificate_ns = start.elapsed().as_nanos();
    let start = Instant::now();
    let native = certificate.emit_rust();
    let emit_ns = start.elapsed().as_nanos();
    let start = Instant::now();
    fs::create_dir(out)?;
    for backend in ["native", "direct", "specialized"] {
        for prereq in [false, true] {
            package(out, root, backend, &native, prereq)?;
        }
    }
    drop(native);
    drop(certificate);
    let package_write_ns = start.elapsed().as_nanos();
    println!(
        "{{\"fixture_ns\":{fixture_ns},\"certificate_ns\":{certificate_ns},\"emit_ns\":{emit_ns},\"package_write_ns\":{package_write_ns},\"total_ns\":{}}}",
        total.elapsed().as_nanos()
    );
    Ok(())
}
