#[path="/home/ahart/Documents/CHRLang/research/chr-compiled/src/recursive/session.rs"] mod session;
use session::Response;
#[path="/home/ahart/Documents/CHRLang/research/chr-compiled/experiments/recursive_session_fixture.rs"] mod fixture;
fn main(){let p=chr_compiled::PreparedRuleset::new(fixture::rules(),None).unwrap().specialize_inferred();session::serve(std::io::stdin().lock(),std::io::stdout().lock(),|q|{
let e=match p.start(q,chr_compiled::Policy::Global,chr_compiled::Access::Indexed){Ok(e)=>e,Err(e)=>return Response::Error(e)};
let mut e=e.into_search();let mut answer=None;
for _ in 0..100_000 {match e.tick(){chr_compiled::SearchEvent::Complete(mut b)=>{if answer.is_some(){return Response::Error("unexpected raw multiplicity".into());}match b.engine.observe(){Some(a)=>answer=Some(a),None=>return Response::Error("observation unavailable".into())}},chr_compiled::SearchEvent::Exhausted=>return answer.map_or(Response::Failure,Response::Success),_=>()}}
Response::Error("service cutoff".into())}).unwrap();}