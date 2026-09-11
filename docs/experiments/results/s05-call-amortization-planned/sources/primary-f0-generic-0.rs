#[allow(dead_code)] #[path="/home/ahart/Documents/CHRLang/research/chr-reuse/examples/call_trace_ownership.rs"] mod lifecycle;
fn main(){let a=std::env::args().collect::<Vec<_>>();assert_eq!(a[2].parse::<usize>().unwrap(),0);lifecycle::entry(None);}
