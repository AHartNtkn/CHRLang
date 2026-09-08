#[path="/home/ahart/Documents/CHRLang/research/chr-compiled/src/recursive/session.rs"] mod session;
use session::Response;
mod generated; fn main(){session::serve(std::io::stdin().lock(),std::io::stdout().lock(),|q|match generated::execute(q){Ok(Some(a))=>Response::Success(a),Ok(None)=>Response::Failure,Err(e)=>Response::Unsupported(e)}).unwrap();}