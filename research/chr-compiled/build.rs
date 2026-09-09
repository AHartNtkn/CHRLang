#[path = "experiments/access_source.rs"]
#[allow(dead_code)]
mod access_source;
#[path = "src/fixtures.rs"]
#[allow(dead_code)]
mod fixtures;
#[path = "src/generate.rs"]
mod generate;
#[path = "src/search_fixtures.rs"]
#[allow(dead_code)]
mod search_fixtures;
#[path = "experiments/subscription_source.rs"]
#[allow(dead_code)]
mod subscription_source;
fn main() {
    println!("cargo:rerun-if-changed=src/generate.rs");
    println!("cargo:rerun-if-changed=src/fixtures.rs");
    println!("cargo:rerun-if-changed=src/search_fixtures.rs");
    println!("cargo:rerun-if-changed=experiments/subscription_source.rs");
    println!("cargo:rerun-if-changed=experiments/access_source.rs");
    let programs = fixtures::programs();
    let mut text = String::from(
        "#[allow(unused_variables,unused_mut)] mod native { use super::{Core,Cursor,Candidate,Selection,Application,Work,Compiled};\n",
    );
    for (id, rules) in programs.iter().enumerate() {
        text.push_str(&generate::emit(&format!("p{id}"), rules).expect("valid source program"));
    }
    text.push_str("}\nfn bundled(id:usize)->Compiled {match id {\n");
    for id in 0..programs.len() {
        text.push_str(&format!("{id}=>native::p{id}_code(),\n"));
    }
    text.push_str("_=>panic!(\"unknown generated program\")}}\n");
    text.push_str("#[allow(unused_variables,unused_mut)] mod search_native {use super::{Core,Cursor,Candidate,Selection,Application,Work,Compiled};\n");
    let search_programs = search_fixtures::programs();
    for (id, rules) in search_programs.iter().enumerate() {
        text.push_str(&generate::emit(&format!("s{id}"), rules).expect("valid search source"));
    }
    text.push_str("}\npub fn search_bundled(id:usize)->Compiled {match id {\n");
    for id in 0..search_programs.len() {
        text.push_str(&format!("{id}=>search_native::s{id}_code(),\n"));
    }
    text.push_str("_=>panic!(\"unknown search program\")}}\n");
    text.push_str("#[allow(unused_variables,unused_mut,unused_labels,non_camel_case_types,dead_code)] mod access_native {use super::{Core,Frame,Selection,Application,Work,Compiled}; use super::native_access::{Continuation,Range};\n");
    for (id, rules) in programs.iter().enumerate() {
        text.push_str(
            &generate::emit_access(&format!("a{id}"), rules).expect("valid access source"),
        );
    }
    text.push_str("}\npub fn access_bundled(id:usize)->Compiled {match id {\n");
    for id in 0..programs.len() {
        text.push_str(&format!("{id}=>access_native::a{id}_code(),\n"));
    }
    text.push_str("_=>panic!(\"unknown access source\")}}\n");
    for (family, sources) in [
        ("search", search_programs),
        ("payload", vec![access_source::payload_rules()]),
        (
            "subscription",
            vec![
                subscription_source::source_rules(false),
                subscription_source::source_rules(true),
            ],
        ),
    ] {
        text.push_str(&format!("#[allow(unused_variables,unused_mut,unused_labels,non_camel_case_types,dead_code)] mod access_{family} {{use super::{{Core,Frame,Selection,Application,Work,Compiled}}; use super::native_access::{{Continuation,Range}};\n"));
        for (id, rules) in sources.iter().enumerate() {
            text.push_str(&generate::emit_access(&format!("a{id}"), rules).expect("valid source"));
        }
        text.push_str(&format!(
            "}}\npub fn access_{family}_bundled(id:usize)->Compiled {{match id {{\n"
        ));
        for id in 0..sources.len() {
            text.push_str(&format!("{id}=>access_{family}::a{id}_code(),\n"));
        }
        text.push_str("_=>panic!(\"unknown source\")}}\n");
    }
    std::fs::write(
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("generated.rs"),
        text,
    )
    .unwrap();
}
