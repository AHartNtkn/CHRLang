#[path = "src/fixtures.rs"]
#[allow(dead_code)]
mod fixtures;
#[path = "src/generate.rs"]
mod generate;
#[path = "src/search_fixtures.rs"]
#[allow(dead_code)]
mod search_fixtures;
fn main() {
    println!("cargo:rerun-if-changed=src/generate.rs");
    println!("cargo:rerun-if-changed=src/fixtures.rs");
    println!("cargo:rerun-if-changed=src/search_fixtures.rs");
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
    std::fs::write(
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("generated.rs"),
        text,
    )
    .unwrap();
}
