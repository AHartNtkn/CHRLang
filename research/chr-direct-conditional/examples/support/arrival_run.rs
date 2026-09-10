use crate::engines::Engine;
pub use super::ordinary::Event;
use chr_syntax::{Query,Rule};
pub const MODES:[&str;6]=["inferred","scan","resumable","specialized","prefix","prepared-prefix"];
#[allow(clippy::large_enum_variant)]
pub enum Prepared {
 Standard(super::ordinary::Prepared),
 Specialized(chr_compiled::PreparedRuleset),
 Prefix { program:chr_compiled::pure_prefix::Program, cache:Option<std::collections::BTreeMap<Vec<(String,usize)>,chr_compiled::pure_prefix::Artifact>> },
}
impl Prepared {
 pub fn new(mode:&str,rules:Vec<Rule>)->Self {
  match mode {
   "specialized"=>Self::Specialized(chr_compiled::PreparedRuleset::new(rules,None).unwrap().specialize_inferred()),
   "prefix"|"prepared-prefix"=>Self::Prefix{program:chr_compiled::pure_prefix::Program::new(&rules).unwrap(),cache:(mode=="prepared-prefix").then(std::collections::BTreeMap::new)},
   _=>Self::Standard(super::ordinary::Prepared::new(mode,rules)),
  }
 }
 pub fn start(&mut self,q:Query)->Engine {
  match self {
   Self::Standard(p)=>p.start(q),
   Self::Specialized(p)=>Engine::Compiled(p.start_search(q,chr_compiled::Policy::Global,chr_compiled::Access::Scan).unwrap()),
   Self::Prefix{program,cache:None}=>{
    let (rules,input)=program.lower(&q).unwrap();
    Engine::Compiled(chr_compiled::PreparedRuleset::new(rules,None).unwrap().start_search(input,chr_compiled::Policy::Global,chr_compiled::Access::Scan).unwrap())
   },
   Self::Prefix{program,cache:Some(cache)}=>{
    let key=q.constraints.iter().map(|c|(c.name.clone(),c.args.len())).collect::<Vec<_>>();
    let artifact=match cache.entry(key){std::collections::btree_map::Entry::Occupied(e)=>e.into_mut(),std::collections::btree_map::Entry::Vacant(e)=>{let a=program.prepare_shape(e.key()).unwrap();e.insert(a)}};
    Engine::Compiled(artifact.start(&q,chr_compiled::Access::Scan).unwrap())
   }
  }
 }
 pub fn artifact_count(&self)->usize {match self{Self::Prefix{cache:Some(c),..}=>c.len(),_=>0}}
 pub fn clear_artifacts(&mut self){if let Self::Prefix{cache:Some(c),..}=self{c.clear()}}
}

