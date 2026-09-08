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
    source.push_str("}\nfn code(id:usize)->chr_compiled::Compiled {match id {0=>native::p0_code(),1=>native::p1_code(),_=>unreachable!()}}\n");
    std::fs::write(
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("generated.rs"),
        source,
    )
    .unwrap();
}
