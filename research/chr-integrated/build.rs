// This build uses one emitter from the shared generator module.
#[allow(dead_code)]
#[path = "../chr-compiled/src/generate.rs"]
mod generate;
#[allow(dead_code)]
#[path = "experiments/workloads.rs"]
mod workloads;
fn main() {
    println!("cargo:rerun-if-changed=../chr-compiled/src/generate.rs");
    println!("cargo:rerun-if-changed=experiments/workloads.rs");
    let mut source = String::from(
        "#[allow(unused_variables,unused_mut)] mod native { use chr_compiled::{Core,Cursor,Candidate,Selection,Application,Work,Compiled};\n",
    );
    for (i, rules) in workloads::programs().iter().enumerate() {
        source.push_str(&generate::emit(&format!("p{i}"), rules).unwrap());
    }
    source.push_str("}\nfn code(id:usize)->chr_compiled::Compiled {match id {\n");
    for i in 0..workloads::programs().len() {
        source.push_str(&format!("{i}=>native::p{i}_code(),\n"));
    }
    source.push_str("_=>unreachable!()}}\n");
    std::fs::write(
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("generated.rs"),
        source,
    )
    .unwrap();
}
