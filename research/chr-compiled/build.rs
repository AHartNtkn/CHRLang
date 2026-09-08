#[path = "src/fixtures.rs"]
#[allow(dead_code)]
mod fixtures;
#[path = "src/generate.rs"]
mod generate;
fn main() {
    println!("cargo:rerun-if-changed=src/generate.rs");
    println!("cargo:rerun-if-changed=src/fixtures.rs");
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
    std::fs::write(
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("generated.rs"),
        text,
    )
    .unwrap();
}
