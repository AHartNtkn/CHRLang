mod generated;
#[allow(dead_code)] #[path="/home/ahart/Documents/CHRLang/research/chr-reuse/tests/call_compiled.rs"] mod gate;
#[allow(dead_code)] #[path="/home/ahart/Documents/CHRLang/research/chr-reuse/examples/call_trace_ownership.rs"] mod lifecycle;
fn main(){let a=std::env::args().collect::<Vec<_>>();if a.get(1).is_some_and(|x|x=="gate"){gate::check(|r,f|Some(generated::code(r,f)));}else{let family=a[2].parse().unwrap();lifecycle::entry(Some(generated::code(false,family)));}}
