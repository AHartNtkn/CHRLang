//! Source-only emission for the matched user-program compilation contrast.
use std::{fs, path::Path, time::Instant};
#[allow(dead_code)]
#[path = "support/call_trace_source.rs"]
mod fixture;
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    assert_eq!(a.len(), 4, "generic|generated FAMILY OUTPUT.rs");
    let native = match &*a[1] {
        "generic" => false,
        "generated" => true,
        _ => panic!("mode"),
    };
    let family: usize = a[2].parse().unwrap();
    assert!(family < 4 && !Path::new(&a[3]).exists());
    let start = Instant::now();
    let rules = fixture::program(false, family).0;
    std::hint::black_box(&rules);
    let generated = if native {
        format!(
            "#[allow(unused_variables,unused_mut,unused_labels,non_camel_case_types,dead_code)] mod generated {{use chr_compiled::{{Core,Frame,Selection,Application,Work,Compiled}};use chr_compiled::native_access::{{Continuation,Range}};\n{}\n}}\n",
            chr_compiled::generate::emit_access("program", &rules).unwrap()
        )
    } else {
        String::new()
    };
    let source = format!(
        "{generated}#[allow(dead_code)] #[path={:?}] mod lifecycle;\nfn main(){{let a=std::env::args().collect::<Vec<_>>();assert_eq!(a[2].parse::<usize>().unwrap(),{family});lifecycle::entry({});}}\n",
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/call_trace_ownership.rs"),
        if native {
            "Some(generated::program_code())"
        } else {
            "None"
        }
    );
    let emission_ns = start.elapsed().as_nanos();
    let start = Instant::now();
    fs::write(&a[3], &source).unwrap();
    let write_ns = start.elapsed().as_nanos();
    let bytes = source.len();
    let start = Instant::now();
    drop((source, generated, rules));
    let buffer_drop_ns = start.elapsed().as_nanos();
    println!(
        "{{\"emission_ns\":{emission_ns},\"write_ns\":{write_ns},\"buffer_drop_ns\":{buffer_drop_ns},\"source_bytes\":{bytes}}}"
    );
}
