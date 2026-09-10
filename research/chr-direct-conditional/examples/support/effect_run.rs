pub use crate::engines::{Engine,Event};
use chr_syntax::{Query,Rule};
pub const MODES:[&str;6]=["ordinary","inferred","declared","required","scan","specialized"];
#[allow(clippy::large_enum_variant)]
pub enum Prepared{Conditional(chr_direct_conditional::engine::PreparedRuleset),Compiled(chr_compiled::PreparedRuleset)}
impl Prepared{
 pub fn try_new(mode:&str,rules:Vec<Rule>)->Result<Self,String>{
  if mode=="scan"||mode=="specialized"{
   let p=chr_compiled::PreparedRuleset::new(rules,None)?;
   return Ok(Self::Compiled(if mode=="specialized"{p.specialize_inferred()}else{p}));
  }
  let p=chr_direct_conditional::engine::PreparedRuleset::with_head_contract(rules,None,chr_direct_conditional::engine::HeadAdmission::Optional)?;
  if mode=="ordinary"{return Ok(Self::Conditional(p));}
  #[cfg(feature="effect-contract")]
  {
   use chr_direct_conditional::engine::{EffectAdmission,EffectDeclaration};
   let declaration=if mode=="inferred"{None}else{Some(EffectDeclaration::NoBindings)};
   let admission=if mode=="required"{EffectAdmission::Required}else{EffectAdmission::Optional};
   Ok(Self::Conditional(p.with_effect_contract(declaration,admission)?))
  }
  #[cfg(not(feature="effect-contract"))]
  Err("effect-contract feature required".into())
 }
 pub fn new(mode:&str,rules:Vec<Rule>)->Self{Self::try_new(mode,rules).unwrap()}
 pub fn start(&self,q:Query)->Engine{match self{
  Self::Conditional(p)=>Engine::Conditional(p.start(q).unwrap()),
  Self::Compiled(p)=>Engine::Compiled(p.start_search(q,chr_compiled::Policy::Global,chr_compiled::Access::Scan).unwrap()),
 }}
}
