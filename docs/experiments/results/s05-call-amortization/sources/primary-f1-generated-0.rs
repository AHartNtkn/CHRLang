#[allow(unused_variables,unused_mut,unused_labels,non_camel_case_types,dead_code)] mod generated {use chr_compiled::{Core,Frame,Selection,Application,Work,Compiled};use chr_compiled::native_access::{Continuation,Range};
#[derive(Clone)] struct program_state_0 { frame:Frame, next:u64, depth:usize, anchor_pending:bool, ids:[u64;1], ranges:[Option<Range>;1] }
fn program_new_0(next:u64)->Box<dyn Continuation>{Box::new(program_state_0{frame:Frame::new(3,next),next,depth:0,anchor_pending:true,ids:[0;1],ranges:[None;1]})}
impl Continuation for program_state_0 { fn duplicate(&self)->Box<dyn Continuation>{Box::new(self.clone())} fn tick(&mut self,core:&mut Core,anchor:Option<(usize,u64)>)->Selection{
match self.depth {
0=>{ self.frame.next=self.next;
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[0]=None;}
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[1]=None;}
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[2]=None;}
if self.ranges[0].is_none() {let mut range=core.access_range(0,0,anchor); if core.access_indexed() && anchor.is_none_or(|(a,_)|a!=0) { 'keys: {
let key='key: {
let Some(t1)=self.frame.slots[0].and_then(|value|core.access_ground(value)) else {break 'key None;};
let t0=core.access_node("s",vec![t1]);
Some(t0)}; if core.access_consider(&mut range,0,key) {break 'keys;}
let key='key: {
let Some(t2)=self.frame.slots[1].and_then(|value|core.access_ground(value)) else {break 'key None;};
Some(t2)}; if core.access_consider(&mut range,1,key) {break 'keys;}
let key='key: {
let Some(t3)=self.frame.slots[2].and_then(|value|core.access_ground(value)) else {break 'key None;};
Some(t3)}; if core.access_consider(&mut range,2,key) {break 'keys;}
} } self.ranges[0]=Some(range);}
let Some(id)=core.access_next(self.ranges[0].as_mut().unwrap()) else {self.ranges[0]=None; return Selection::Done; };
if self.ids[..0].contains(&id) {return Selection::Yield;} let Some(args)=core.arguments(id) else {return Selection::Yield;};
let Some(t4)=core.constructor(args[0],"s",1) else {return Selection::Yield;};
if !core.bind(&mut self.frame, 0, t4[0]) {return Selection::Yield;}
if !core.bind(&mut self.frame, 1, args[1]) {return Selection::Yield;}
if !core.bind(&mut self.frame, 2, args[2]) {return Selection::Yield;}
self.ids[0]=id; self.depth=1; Selection::Yield },
1=>{if !core.eligible(0,&self.ids){self.depth=0;return Selection::Yield;}
let t6=core.variable(&mut self.frame,0);
let t7=core.variable(&mut self.frame,1);
let t8=core.variable(&mut self.frame,2);
let t5=Work::Insert(core.body_predicate(0,0),vec![t6,t7,t8]);
Selection::Found(Application{rule:0,ids:self.ids.to_vec(),body:t5,next:self.frame.next}) }, _=>unreachable!(),} } }
#[derive(Clone)] struct program_state_1 { frame:Frame, next:u64, depth:usize, anchor_pending:bool, ids:[u64;1], ranges:[Option<Range>;1] }
fn program_new_1(next:u64)->Box<dyn Continuation>{Box::new(program_state_1{frame:Frame::new(1,next),next,depth:0,anchor_pending:true,ids:[0;1],ranges:[None;1]})}
impl Continuation for program_state_1 { fn duplicate(&self)->Box<dyn Continuation>{Box::new(self.clone())} fn tick(&mut self,core:&mut Core,anchor:Option<(usize,u64)>)->Selection{
match self.depth {
0=>{ self.frame.next=self.next;
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[0]=None;}
if self.ranges[0].is_none() {let mut range=core.access_range(1,0,anchor); if core.access_indexed() && anchor.is_none_or(|(a,_)|a!=0) { 'keys: {
let key='key: {
let t0=core.access_node("z",vec![]);
Some(t0)}; if core.access_consider(&mut range,0,key) {break 'keys;}
let key='key: {
let t1=core.access_node("b",vec![]);
Some(t1)}; if core.access_consider(&mut range,1,key) {break 'keys;}
let key='key: {
let Some(t2)=self.frame.slots[0].and_then(|value|core.access_ground(value)) else {break 'key None;};
Some(t2)}; if core.access_consider(&mut range,2,key) {break 'keys;}
} } self.ranges[0]=Some(range);}
let Some(id)=core.access_next(self.ranges[0].as_mut().unwrap()) else {self.ranges[0]=None; return Selection::Done; };
if self.ids[..0].contains(&id) {return Selection::Yield;} let Some(args)=core.arguments(id) else {return Selection::Yield;};
let Some(t3)=core.constructor(args[0],"z",0) else {return Selection::Yield;};
let Some(t4)=core.constructor(args[1],"b",0) else {return Selection::Yield;};
if !core.bind(&mut self.frame, 0, args[2]) {return Selection::Yield;}
self.ids[0]=id; self.depth=1; Selection::Yield },
1=>{if !core.eligible(1,&self.ids){self.depth=0;return Selection::Yield;}
let t5=Work::Fail;
Selection::Found(Application{rule:1,ids:self.ids.to_vec(),body:t5,next:self.frame.next}) }, _=>unreachable!(),} } }
#[derive(Clone)] struct program_state_2 { frame:Frame, next:u64, depth:usize, anchor_pending:bool, ids:[u64;1], ranges:[Option<Range>;1] }
fn program_new_2(next:u64)->Box<dyn Continuation>{Box::new(program_state_2{frame:Frame::new(2,next),next,depth:0,anchor_pending:true,ids:[0;1],ranges:[None;1]})}
impl Continuation for program_state_2 { fn duplicate(&self)->Box<dyn Continuation>{Box::new(self.clone())} fn tick(&mut self,core:&mut Core,anchor:Option<(usize,u64)>)->Selection{
match self.depth {
0=>{ self.frame.next=self.next;
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[0]=None;}
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[1]=None;}
if self.ranges[0].is_none() {let mut range=core.access_range(2,0,anchor); if core.access_indexed() && anchor.is_none_or(|(a,_)|a!=0) { 'keys: {
let key='key: {
let t0=core.access_node("z",vec![]);
Some(t0)}; if core.access_consider(&mut range,0,key) {break 'keys;}
let key='key: {
let Some(t1)=self.frame.slots[0].and_then(|value|core.access_ground(value)) else {break 'key None;};
Some(t1)}; if core.access_consider(&mut range,1,key) {break 'keys;}
let key='key: {
let Some(t2)=self.frame.slots[1].and_then(|value|core.access_ground(value)) else {break 'key None;};
Some(t2)}; if core.access_consider(&mut range,2,key) {break 'keys;}
} } self.ranges[0]=Some(range);}
let Some(id)=core.access_next(self.ranges[0].as_mut().unwrap()) else {self.ranges[0]=None; return Selection::Done; };
if self.ids[..0].contains(&id) {return Selection::Yield;} let Some(args)=core.arguments(id) else {return Selection::Yield;};
let Some(t3)=core.constructor(args[0],"z",0) else {return Selection::Yield;};
if !core.bind(&mut self.frame, 0, args[1]) {return Selection::Yield;}
if !core.bind(&mut self.frame, 1, args[2]) {return Selection::Yield;}
self.ids[0]=id; self.depth=1; Selection::Yield },
1=>{if !core.eligible(2,&self.ids){self.depth=0;return Selection::Yield;}
let t5=core.variable(&mut self.frame,1);
let t6=core.variable(&mut self.frame,0);
let t4=Work::Equal(t5,t6);
Selection::Found(Application{rule:2,ids:self.ids.to_vec(),body:t4,next:self.frame.next}) }, _=>unreachable!(),} } }
#[derive(Clone)] struct program_state_3 { frame:Frame, next:u64, depth:usize, anchor_pending:bool, ids:[u64;1], ranges:[Option<Range>;1] }
fn program_new_3(next:u64)->Box<dyn Continuation>{Box::new(program_state_3{frame:Frame::new(1,next),next,depth:0,anchor_pending:true,ids:[0;1],ranges:[None;1]})}
impl Continuation for program_state_3 { fn duplicate(&self)->Box<dyn Continuation>{Box::new(self.clone())} fn tick(&mut self,core:&mut Core,anchor:Option<(usize,u64)>)->Selection{
match self.depth {
0=>{ self.frame.next=self.next;
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[0]=None;}
if self.ranges[0].is_none() {let mut range=core.access_range(3,0,anchor); if core.access_indexed() && anchor.is_none_or(|(a,_)|a!=0) { 'keys: {
let key='key: {
let Some(t0)=self.frame.slots[0].and_then(|value|core.access_ground(value)) else {break 'key None;};
Some(t0)}; if core.access_consider(&mut range,0,key) {break 'keys;}
} } self.ranges[0]=Some(range);}
let Some(id)=core.access_next(self.ranges[0].as_mut().unwrap()) else {self.ranges[0]=None; return Selection::Done; };
if self.ids[..0].contains(&id) {return Selection::Yield;} let Some(args)=core.arguments(id) else {return Selection::Yield;};
if !core.bind(&mut self.frame, 0, args[0]) {return Selection::Yield;}
self.ids[0]=id; self.depth=1; Selection::Yield },
1=>{if !core.eligible(3,&self.ids){self.depth=0;return Selection::Yield;}
let t2=core.variable(&mut self.frame,0);
let t1=Work::Insert(core.body_predicate(3,0),vec![t2]);
Selection::Found(Application{rule:3,ids:self.ids.to_vec(),body:t1,next:self.frame.next}) }, _=>unreachable!(),} } }
#[derive(Clone)] struct program_state_4 { frame:Frame, next:u64, depth:usize, anchor_pending:bool, ids:[u64;1], ranges:[Option<Range>;1] }
fn program_new_4(next:u64)->Box<dyn Continuation>{Box::new(program_state_4{frame:Frame::new(3,next),next,depth:0,anchor_pending:true,ids:[0;1],ranges:[None;1]})}
impl Continuation for program_state_4 { fn duplicate(&self)->Box<dyn Continuation>{Box::new(self.clone())} fn tick(&mut self,core:&mut Core,anchor:Option<(usize,u64)>)->Selection{
match self.depth {
0=>{ self.frame.next=self.next;
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[0]=None;}
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[1]=None;}
if !matches!(anchor.map(|(h,_)|h),Some(0)) {self.frame.slots[2]=None;}
if self.ranges[0].is_none() {let mut range=core.access_range(4,0,anchor); if core.access_indexed() && anchor.is_none_or(|(a,_)|a!=0) { 'keys: {
let key='key: {
let Some(t0)=self.frame.slots[0].and_then(|value|core.access_ground(value)) else {break 'key None;};
Some(t0)}; if core.access_consider(&mut range,0,key) {break 'keys;}
let key='key: {
let Some(t1)=self.frame.slots[1].and_then(|value|core.access_ground(value)) else {break 'key None;};
Some(t1)}; if core.access_consider(&mut range,1,key) {break 'keys;}
let key='key: {
let Some(t2)=self.frame.slots[2].and_then(|value|core.access_ground(value)) else {break 'key None;};
Some(t2)}; if core.access_consider(&mut range,2,key) {break 'keys;}
} } self.ranges[0]=Some(range);}
let Some(id)=core.access_next(self.ranges[0].as_mut().unwrap()) else {self.ranges[0]=None; return Selection::Done; };
if self.ids[..0].contains(&id) {return Selection::Yield;} let Some(args)=core.arguments(id) else {return Selection::Yield;};
if !core.bind(&mut self.frame, 0, args[0]) {return Selection::Yield;}
if !core.bind(&mut self.frame, 1, args[1]) {return Selection::Yield;}
if !core.bind(&mut self.frame, 2, args[2]) {return Selection::Yield;}
self.ids[0]=id; self.depth=1; Selection::Yield },
1=>{if !core.eligible(4,&self.ids){self.depth=0;return Selection::Yield;}
let t5=core.variable(&mut self.frame,0);
let t6=core.make("a",vec![]);
let t7=core.variable(&mut self.frame,2);
let t4=Work::Insert(core.body_predicate(4,0),vec![t5,t6,t7]);
let t9=core.variable(&mut self.frame,1);
let t10=core.make("b",vec![]);
let t11=core.variable(&mut self.frame,2);
let t8=Work::Insert(core.body_predicate(4,1),vec![t9,t10,t11]);
let t3=Work::Or(Box::new(t4),Box::new(t8));
Selection::Found(Application{rule:4,ids:self.ids.to_vec(),body:t3,next:self.frame.next}) }, _=>unreachable!(),} } }
#[derive(Clone)] struct program_state_5 { frame:Frame, next:u64, depth:usize, anchor_pending:bool, ids:[u64;2], ranges:[Option<Range>;2] }
fn program_new_5(next:u64)->Box<dyn Continuation>{Box::new(program_state_5{frame:Frame::new(0,next),next,depth:0,anchor_pending:true,ids:[0;2],ranges:[None;2]})}
impl Continuation for program_state_5 { fn duplicate(&self)->Box<dyn Continuation>{Box::new(self.clone())} fn tick(&mut self,core:&mut Core,anchor:Option<(usize,u64)>)->Selection{
if self.anchor_pending {self.anchor_pending=false; if let Some((head,id))=anchor.filter(|(h,_)| *h>0) {let Some(args)=core.arguments(id) else {return Selection::Done;}; match head {
1=>{
},
_=>unreachable!(),} return Selection::Yield;}}
match self.depth {
0=>{ self.frame.next=self.next;
if self.ranges[0].is_none() {let mut range=core.access_range(5,0,anchor); if core.access_indexed() && anchor.is_none_or(|(a,_)|a!=0) { 'keys: {
let key='key: {
let t0=core.access_node("a",vec![]);
Some(t0)}; if core.access_consider(&mut range,0,key) {break 'keys;}
} } self.ranges[0]=Some(range);}
let Some(id)=core.access_next(self.ranges[0].as_mut().unwrap()) else {self.ranges[0]=None; return Selection::Done; };
if self.ids[..0].contains(&id) {return Selection::Yield;} let Some(args)=core.arguments(id) else {return Selection::Yield;};
let Some(t1)=core.constructor(args[0],"a",0) else {return Selection::Yield;};
self.ids[0]=id; self.depth=1; Selection::Yield },
1=>{ self.frame.next=self.next;
if self.ranges[1].is_none() {let mut range=core.access_range(5,1,anchor); if core.access_indexed() && anchor.is_none_or(|(a,_)|a!=1) { 'keys: {
} } self.ranges[1]=Some(range);}
let Some(id)=core.access_next(self.ranges[1].as_mut().unwrap()) else {self.ranges[1]=None; self.depth=0;return Selection::Yield; };
if self.ids[..1].contains(&id) {return Selection::Yield;} let Some(args)=core.arguments(id) else {return Selection::Yield;};
self.ids[1]=id; self.depth=2; Selection::Yield },
2=>{if !core.eligible(5,&self.ids){self.depth=1;return Selection::Yield;}
let t3=core.make("a",vec![]);
let t2=Work::Insert(core.body_predicate(5,0),vec![t3]);
Selection::Found(Application{rule:5,ids:self.ids.to_vec(),body:t2,next:self.frame.next}) }, _=>unreachable!(),} } }
#[derive(Clone)] struct program_state_6 { frame:Frame, next:u64, depth:usize, anchor_pending:bool, ids:[u64;1], ranges:[Option<Range>;1] }
fn program_new_6(next:u64)->Box<dyn Continuation>{Box::new(program_state_6{frame:Frame::new(0,next),next,depth:0,anchor_pending:true,ids:[0;1],ranges:[None;1]})}
impl Continuation for program_state_6 { fn duplicate(&self)->Box<dyn Continuation>{Box::new(self.clone())} fn tick(&mut self,core:&mut Core,anchor:Option<(usize,u64)>)->Selection{
match self.depth {
0=>{ self.frame.next=self.next;
if self.ranges[0].is_none() {let mut range=core.access_range(6,0,anchor); if core.access_indexed() && anchor.is_none_or(|(a,_)|a!=0) { 'keys: {
} } self.ranges[0]=Some(range);}
let Some(id)=core.access_next(self.ranges[0].as_mut().unwrap()) else {self.ranges[0]=None; return Selection::Done; };
if self.ids[..0].contains(&id) {return Selection::Yield;} let Some(args)=core.arguments(id) else {return Selection::Yield;};
self.ids[0]=id; self.depth=1; Selection::Yield },
1=>{if !core.eligible(6,&self.ids){self.depth=0;return Selection::Yield;}
let t1=core.make("other",vec![]);
let t0=Work::Insert(core.body_predicate(6,0),vec![t1]);
Selection::Found(Application{rule:6,ids:self.ids.to_vec(),body:t0,next:self.frame.next}) }, _=>unreachable!(),} } }
fn program_update_0(core:&mut Core,id:u64){let mut work=core.update_start();
if core.update_active() {core.update_watch(id,0,&mut work);}
if core.update_active() {core.update_watch(id,1,&mut work);}
if core.update_active() {core.update_watch(id,2,&mut work);}
core.update_finish(id,work);}
fn program_update_1(core:&mut Core,id:u64){let mut work=core.update_start();
core.update_finish(id,work);}
fn program_update_2(core:&mut Core,id:u64){let mut work=core.update_start();
if core.update_active() || core.access_indexed() {core.update_watch(id,0,&mut work);}
if core.access_indexed() {core.update_index(id,0,&mut work);}
if core.update_active() || core.access_indexed() {core.update_watch(id,1,&mut work);}
if core.access_indexed() {core.update_index(id,1,&mut work);}
if core.update_active() {core.update_watch(id,2,&mut work);}
core.update_finish(id,work);}
fn program_update_3(core:&mut Core,id:u64){let mut work=core.update_start();
if core.update_active() || core.access_indexed() {core.update_watch(id,0,&mut work);}
if core.access_indexed() {core.update_index(id,0,&mut work);}
core.update_finish(id,work);}
pub fn program_code()->Compiled{Compiled{source:"[Rule { name: \"recurse\", kept: [], removed: [Constraint { name: \"wait\", args: [App(\"s\", [Var(Var(0))]), Var(Var(1)), Var(Var(2))] }], guards: [], body: Constraint(Constraint { name: \"wait\", args: [Var(Var(0)), Var(Var(1)), Var(Var(2))] }) }, Rule { name: \"fail_b\", kept: [], removed: [Constraint { name: \"wait\", args: [App(\"z\", []), App(\"b\", []), Var(Var(0))] }], guards: [], body: Fail }, Rule { name: \"base\", kept: [], removed: [Constraint { name: \"wait\", args: [App(\"z\", []), Var(Var(0)), Var(Var(1))] }], guards: [], body: Unify(Var(Var(1)), Var(Var(0))) }, Rule { name: \"history\", kept: [Constraint { name: \"watch\", args: [Var(Var(0))] }], removed: [], guards: [], body: Constraint(Constraint { name: \"seen\", args: [Var(Var(0))] }) }, Rule { name: \"launch\", kept: [], removed: [Constraint { name: \"start\", args: [Var(Var(0)), Var(Var(1)), Var(Var(2))] }], guards: [], body: Or(Constraint(Constraint { name: \"wait\", args: [Var(Var(0)), App(\"a\", []), Var(Var(2))] }), Constraint(Constraint { name: \"wait\", args: [Var(Var(1)), App(\"b\", []), Var(Var(2))] })) }, Rule { name: \"a_wins\", kept: [], removed: [Constraint { name: \"watch\", args: [App(\"a\", [])] }, Constraint { name: \"token\", args: [] }], guards: [], body: Constraint(Constraint { name: \"winner\", args: [App(\"a\", [])] }) }, Rule { name: \"other_wins\", kept: [], removed: [Constraint { name: \"token\", args: [] }], guards: [], body: Constraint(Constraint { name: \"winner\", args: [App(\"other\", [])] }) }]",selectors:&[],native:Some(&[program_new_0,program_new_1,program_new_2,program_new_3,program_new_4,program_new_5,program_new_6]),updates:Some(&[("start",3,program_update_0),("token",0,program_update_1),("wait",3,program_update_2),("watch",1,program_update_3)])}}

}
#[allow(dead_code)] #[path="/home/ahart/Documents/CHRLang/research/chr-reuse/examples/call_trace_ownership.rs"] mod lifecycle;
fn main(){let a=std::env::args().collect::<Vec<_>>();assert_eq!(a[2].parse::<usize>().unwrap(),1);lifecycle::entry(Some(generated::program_code()));}
