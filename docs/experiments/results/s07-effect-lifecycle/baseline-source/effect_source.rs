use chr_syntax::{Answer,Goal,Query,Rule,Term,Var,atom,c,eq,t,v};
#[derive(Clone,Copy)]
pub struct Schema{pub family:&'static str,pub work:usize,pub payload:usize,pub resource:bool,pub fail_tail:bool}
fn unary(name:&str,end:&str,n:usize)->Term{(0..n).fold(atom(end),|a,_|t(name,[a]))}
impl Schema{
 pub fn rules(self)->Vec<Rule>{
  let mut rules=vec![Rule::simplify("match",[c("p",[atom("a")])],c("done",[]).into())];
  match self.family{
   "blocked"=>(),
   "writer"=>rules.push(Rule::simplify("bind",[c("bind",[v(0)])],eq(v(0),atom("a")))),
   "rewrite"=>{
    rules.push(Rule::simplify("step",[c("walk",[t("s",[v(0)]),v(1),v(2)])],c("walk",[v(0),v(1),t("wrap",[v(2)])]).into()));
    rules.push(Rule::simplify("end",[c("walk",[atom("z"),v(0),v(1)])],c("result",[v(0),v(1)]).into()));
    let mut heads=vec![c("result",[v(0),v(1)])];if self.resource{heads.push(c("token",[]));}
    rules.push(Rule::simplify("finish",heads,c("out",[v(0),v(1)]).into()));
   },
   _=>panic!("family"),
  }
  if self.fail_tail{rules.push(Rule::simplify("fail",[c("die",[])],Goal::Fail));}
  rules
 }
 pub fn query(self,n:usize,reverse:bool)->Query{
  let mut constraints=vec![c("p",[v(100)]);n];
  if self.family=="writer"{constraints.push(c("bind",[v(100)]));}
  if self.family=="rewrite"{
   constraints.push(c("walk",[unary("s","z",n*self.work),v(100),unary("data","end",self.payload)]));
   if self.resource{constraints.push(c("token",[]));}
  }
  if self.fail_tail{constraints.push(c("die",[]));}
  constraints.push(c("tag",[v(100),atom(if reverse{"second"}else{"first"})]));
  if reverse{constraints.reverse();}
  Query{constraints,outputs:vec![("x".into(),Var(100)),("again".into(),Var(100))]}
 }
 pub fn expected_query(self,n:usize,reverse:bool)->Vec<Answer>{
  if self.fail_tail{return vec![];}
  let x=if self.family=="writer"{atom("a")}else{v(0)};
  let mut residual=if self.family=="writer"{vec![c("done",[]);n]}else{vec![c("p",[x.clone()]);n]};
  residual.push(c("tag",[x.clone(),atom(if reverse{"second"}else{"first"})]));
  if self.family=="rewrite"{
   let data=(0..n*self.work).fold(unary("data","end",self.payload),|a,_|t("wrap",[a]));
   residual.push(c("out",[x.clone(),data]));
  }
  vec![Answer{outputs:vec![("x".into(),x.clone()),("again".into(),x)],residual}]
 }
}
