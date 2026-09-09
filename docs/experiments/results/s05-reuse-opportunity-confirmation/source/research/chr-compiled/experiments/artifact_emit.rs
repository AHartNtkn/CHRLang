//! Source-only artifact emission. Query construction is in the shared runner.
use std::{fs, time::Instant};
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 {
        return Err("usage: generic|chain|payload|subscription OUTPUT.rs".into());
    }
    let start = Instant::now();
    let code = if args[0] == "generic" {
        "fn main(){chr_compiled::artifact_runtime::entry(None);}\n".to_string()
    } else {
        let rules = chr_compiled::artifact_runtime::rules(&args[0])?;
        let generated = chr_compiled::generate::emit_access("program", &rules)?;
        format!(
            "#[allow(unused_variables,unused_mut,unused_labels,non_camel_case_types,dead_code)] mod generated {{ use chr_compiled::{{Core,Frame,Selection,Application,Work,Compiled}}; use chr_compiled::native_access::{{Continuation,Range}};\n{generated}\n}}\nfn main(){{chr_compiled::artifact_runtime::entry(Some(generated::program_code()));}}\n"
        )
    };
    let generation_ns = start.elapsed().as_nanos();
    let start = Instant::now();
    fs::write(&args[1], &code).map_err(|e| e.to_string())?;
    let write_ns = start.elapsed().as_nanos();
    let source_bytes = code.len();
    let start = Instant::now();
    drop(code);
    let buffer_drop_ns = start.elapsed().as_nanos();
    println!(
        "{{\"family\":{:?},\"generation_ns\":{generation_ns},\"source_write_ns\":{write_ns},\"buffer_drop_ns\":{buffer_drop_ns},\"source_bytes\":{}}}",
        args[0], source_bytes
    );
    Ok(())
}
