#[path="/home/ahart/Documents/CHRLang/research/chr-compiled/src/recursive/session.rs"] mod session;
use session::Response;
#[path="/home/ahart/Documents/CHRLang/research/chr-compiled/experiments/recursive_session_fixture.rs"] mod fixture;
fn main(){let p=chr_compiled::recursive::Prepared::new(fixture::rules()).unwrap();session::serve(std::io::stdin().lock(),std::io::stdout().lock(),|q|match p.execute(q){Ok(Some(a))=>Response::Success(a),Ok(None)=>Response::Failure,Err(e)=>Response::Unsupported(e.0)}).unwrap();}