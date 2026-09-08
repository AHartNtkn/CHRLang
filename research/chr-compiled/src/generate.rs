//! Emits native Rust for rules, independent of query depth or query values.
use chr_syntax::{Goal, Guard, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
fn collect_term(t: &Term, vars: &mut BTreeSet<u64>) {
    match t {
        Term::Var(Var(v)) => {
            vars.insert(*v);
        }
        Term::App(_, args) => {
            for a in args {
                collect_term(a, vars)
            }
        }
    }
}
fn collect_goal(g: &Goal, vars: &mut BTreeSet<u64>) -> Result<(), String> {
    match g {
        Goal::Constraint(c) => {
            for t in &c.args {
                collect_term(t, vars)
            }
        }
        Goal::Unify(a, b) => {
            collect_term(a, vars);
            collect_term(b, vars)
        }
        Goal::And(gs) => {
            for g in gs {
                collect_goal(g, vars)?
            }
        }
        Goal::True | Goal::Fail => (),
        Goal::Or(a, b) => {
            collect_goal(a, vars)?;
            collect_goal(b, vars)?;
        }
    }
    Ok(())
}
struct Emit {
    text: String,
    next: usize,
    slots: BTreeMap<u64, usize>,
    rule: usize,
    body_pred: usize,
}
impl Emit {
    fn name(&mut self) -> String {
        let n = format!("t{}", self.next);
        self.next += 1;
        n
    }
    fn pattern(&mut self, p: &Term, value: &str, frame: &str, failure: &str) {
        match p {
            Term::Var(Var(v)) => {
                writeln!(
                    self.text,
                    "if !core.bind(&mut {frame}, {}, {value}) {{return {failure};}}",
                    self.slots[v]
                )
                .unwrap();
            }
            Term::App(name, args) => {
                let temp = self.name();
                writeln!(self.text,"let Some({temp})=core.constructor({value},{name:?},{}) else {{return {failure};}};",args.len()).unwrap();
                for (i, p) in args.iter().enumerate() {
                    self.pattern(p, &format!("{temp}[{i}]"), frame, failure)
                }
            }
        }
    }
    fn term(&mut self, t: &Term, frame: &str) -> String {
        let temp = self.name();
        match t {
            Term::Var(Var(v)) => writeln!(
                self.text,
                "let {temp}=core.variable(&mut {frame},{});",
                self.slots[v]
            )
            .unwrap(),
            Term::App(name, args) => {
                let args = args
                    .iter()
                    .map(|a| self.term(a, frame))
                    .collect::<Vec<_>>()
                    .join(",");
                writeln!(self.text, "let {temp}=core.make({name:?},vec![{args}]);").unwrap();
            }
        }
        temp
    }
    fn goal(&mut self, g: &Goal, frame: &str) -> String {
        let temp = self.name();
        let expr = match g {
            Goal::Constraint(c) => {
                let args = c
                    .args
                    .iter()
                    .map(|a| self.term(a, frame))
                    .collect::<Vec<_>>()
                    .join(",");
                {
                    let pred = self.body_pred;
                    self.body_pred += 1;
                    format!(
                        "Work::Insert(core.body_predicate({},{}),vec![{args}])",
                        self.rule, pred
                    )
                }
            }
            Goal::Unify(a, b) => {
                let a = self.term(a, frame);
                let b = self.term(b, frame);
                format!("Work::Equal({a},{b})")
            }
            Goal::And(gs) => {
                let gs = gs
                    .iter()
                    .map(|g| self.goal(g, frame))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("Work::And(vec![{gs}])")
            }
            Goal::True => "Work::True".into(),
            Goal::Fail => "Work::Fail".into(),
            Goal::Or(a, b) => {
                let a = self.goal(a, frame);
                let b = self.goal(b, frame);
                format!("Work::Or(Box::new({a}),Box::new({b}))")
            }
        };
        writeln!(self.text, "let {temp}={expr};").unwrap();
        temp
    }
}
fn validate_prefix(name: &str) -> Result<(), String> {
    if name.is_empty()
        || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
        || name.as_bytes()[0].is_ascii_digit()
    {
        return Err("invalid Rust prefix".into());
    }
    Ok(())
}
/// The result defines `{name}_code() -> Compiled` and native rule selectors.
/// Include inside a module importing Core, Cursor, Candidate, Selection, Application, Work and Compiled.
pub fn emit(name: &str, rules: &[Rule]) -> Result<String, String> {
    validate_prefix(name)?;
    let mut text = String::new();
    let mut funcs = vec![];
    for (r, rule) in rules.iter().enumerate() {
        let heads = rule.kept.iter().chain(&rule.removed).collect::<Vec<_>>();
        if heads.is_empty() {
            return Err("empty heads".into());
        }
        let mut vars = BTreeSet::new();
        for h in &heads {
            for t in &h.args {
                collect_term(t, &mut vars)
            }
        }
        for Guard::Equal(a, b) in &rule.guards {
            collect_term(a, &mut vars);
            collect_term(b, &mut vars)
        }
        collect_goal(&rule.body, &mut vars)?;
        let mut e = Emit {
            text: String::new(),
            next: 0,
            slots: vars.into_iter().enumerate().map(|(i, v)| (v, i)).collect(),
            rule: r,
            body_pred: 0,
        };
        let func = format!("{name}_r{r}");
        funcs.push(func.clone());
        writeln!(
            e.text,
            "fn {func}(core:&mut Core,cursor:&mut Cursor,anchor:Option<(usize,u64)>)->Selection{{"
        )
        .unwrap();
        if heads.len() > 1 {
            e.text.push_str("if cursor.anchor_pending { cursor.anchor_pending=false; if let Some((head,id))=anchor.filter(|(head,_)| *head>0) { let Some(args)=core.arguments(id) else {return Selection::Done;}; let mut frame=core.copy_frame(&cursor.frames[0]); match head {\n");
            for (h, head) in heads.iter().enumerate().skip(1) {
                writeln!(e.text, "{h} => {{").unwrap();
                for (a, p) in head.args.iter().enumerate() {
                    e.pattern(p, &format!("args[{a}]"), "frame", "Selection::Done");
                }
                e.text.push_str("},\n");
            }
            e.text.push_str("_=>unreachable!(\"invalid anchor head\"),} cursor.frames[0]=frame; return Selection::Yield; }}\n");
        }
        e.text.push_str("match cursor.depth {\n");
        for (h, head) in heads.iter().enumerate() {
            writeln!(e.text,"{h} => {{\nlet id=match core.candidate({r},{h},anchor,cursor){{Candidate::Value(id)=>id,Candidate::Yield=>return Selection::Yield,Candidate::Done=>return Selection::Done}};\nlet Some(args)=core.arguments(id) else{{return Selection::Yield}};\nlet mut frame=core.copy_frame(&cursor.frames[{h}]);").unwrap();
            for (a, p) in head.args.iter().enumerate() {
                e.pattern(p, &format!("args[{a}]"), "frame", "Selection::Yield")
            }
            e.text
                .push_str("cursor.descend(id,frame); Selection::Yield\n},\n");
        }
        let h = heads.len();
        writeln!(e.text,"{h} => {{\nif !core.eligible({r},&cursor.ids){{cursor.backtrack();return Selection::Yield}}\nlet mut frame=core.copy_frame(&cursor.frames[{h}]);").unwrap();
        for Guard::Equal(a, b) in &rule.guards {
            let a = e.term(a, "frame");
            let b = e.term(b, "frame");
            writeln!(
                e.text,
                "if !core.equal({a},{b}){{cursor.backtrack();return Selection::Yield}}"
            )
            .unwrap();
        }
        let body = e.goal(&rule.body, "frame");
        writeln!(e.text,"Selection::Found(Application{{rule:{r},ids:cursor.ids.clone(),body:{body},next:frame.next}})\n}},\n_=>unreachable!(\"invalid rule cursor\"),\n}}\n}}").unwrap();
        text.push_str(&e.text);
    }
    writeln!(
        text,
        "pub fn {name}_code()->Compiled{{Compiled{{source:{:?},selectors:&[{}],native:None}}}}",
        format!("{rules:?}"),
        funcs.join(",")
    )
    .unwrap();
    Ok(text)
}

impl Emit {
    /// A key exists only when every source-pattern component is established.
    fn access_key(&mut self, term: &Term) -> String {
        let temp = self.name();
        match term {
            Term::Var(Var(v)) => {
                writeln!(self.text,"let Some({temp})=self.frame.slots[{}].and_then(|value|core.access_ground(value)) else {{break 'key None;}};",self.slots[v]).unwrap();
            }
            Term::App(name, args) => {
                let children = args
                    .iter()
                    .map(|a| self.access_key(a))
                    .collect::<Vec<_>>()
                    .join(",");
                writeln!(
                    self.text,
                    "let {temp}=core.access_node({name:?},vec![{children}]);"
                )
                .unwrap();
            }
        }
        temp
    }
}

/// Emit source-ordered native continuations without frame snapshots or pools.
/// In addition to the ordinary generator imports, import Frame, Continuation,
/// and Range from the runtime. Code and state dimensions depend only on rules.
pub fn emit_access(name: &str, rules: &[Rule]) -> Result<String, String> {
    validate_prefix(name)?;
    let mut text = String::new();
    let mut factories = Vec::new();
    for (r, rule) in rules.iter().enumerate() {
        let heads: Vec<_> = rule.kept.iter().chain(&rule.removed).collect();
        let mut vars = BTreeSet::new();
        let mut head_vars = Vec::new();
        for head in &heads {
            let mut local = BTreeSet::new();
            for term in &head.args {
                collect_term(term, &mut local);
            }
            vars.extend(&local);
            head_vars.push(local);
        }
        for Guard::Equal(a, b) in &rule.guards {
            collect_term(a, &mut vars);
            collect_term(b, &mut vars);
        }
        collect_goal(&rule.body, &mut vars)?;
        let slots = vars.len();
        let hcount = heads.len();
        if hcount == 0 {
            return Err("empty heads".into());
        }
        let mut e = Emit {
            text: String::new(),
            next: 0,
            slots: vars.into_iter().enumerate().map(|(i, v)| (v, i)).collect(),
            rule: r,
            body_pred: 0,
        };
        let state = format!("{name}_state_{r}");
        let factory = format!("{name}_new_{r}");
        factories.push(factory.clone());
        writeln!(e.text,"#[derive(Clone)] struct {state} {{ frame:Frame, next:u64, depth:usize, anchor_pending:bool, ids:[u64;{hcount}], ranges:[Option<Range>;{hcount}] }}").unwrap();
        writeln!(e.text,"fn {factory}(next:u64)->Box<dyn Continuation>{{Box::new({state}{{frame:Frame::new({slots},next),next,depth:0,anchor_pending:true,ids:[0;{hcount}],ranges:[None;{hcount}]}})}}").unwrap();
        writeln!(e.text,"impl Continuation for {state} {{ fn duplicate(&self)->Box<dyn Continuation>{{Box::new(self.clone())}} fn tick(&mut self,core:&mut Core,anchor:Option<(usize,u64)>)->Selection{{").unwrap();
        if heads.len() > 1 {
            e.text.push_str("if self.anchor_pending {self.anchor_pending=false; if let Some((head,id))=anchor.filter(|(h,_)| *h>0) {let Some(args)=core.arguments(id) else {return Selection::Done;}; match head {\n");
            for (h, head) in heads.iter().enumerate().skip(1) {
                writeln!(e.text, "{h}=>{{").unwrap();
                for (a, p) in head.args.iter().enumerate() {
                    e.pattern(p, &format!("args[{a}]"), "self.frame", "Selection::Done");
                }
                e.text.push_str("},\n");
            }
            e.text
                .push_str("_=>unreachable!(),} return Selection::Yield;}}\n");
        }
        e.text.push_str("match self.depth {\n");
        let mut prefix: BTreeSet<u64> = BTreeSet::new();
        for (h, head) in heads.iter().enumerate() {
            writeln!(e.text, "{h}=>{{ self.frame.next=self.next;").unwrap();
            // Slots not justified by the retained prefix must be cleared even
            // after a partially successful pattern or a failed guard.
            for (&v, &slot) in &e.slots {
                if prefix.contains(&v) {
                    continue;
                }
                let preserve: Vec<_> = head_vars
                    .iter()
                    .enumerate()
                    .filter(|(_, vs)| vs.contains(&v))
                    .map(|(h, _)| format!("Some({h})"))
                    .collect();
                if preserve.is_empty() {
                    writeln!(e.text, "self.frame.slots[{slot}]=None;").unwrap();
                } else {
                    writeln!(
                        e.text,
                        "if !matches!(anchor.map(|(h,_)|h),{}) {{self.frame.slots[{slot}]=None;}}",
                        preserve.join("|")
                    )
                    .unwrap();
                }
            }
            writeln!(e.text,"if self.ranges[{h}].is_none() {{let mut range=core.access_range({r},{h},anchor); if core.access_indexed() && anchor.is_none_or(|(a,_)|a!={h}) {{ 'keys: {{").unwrap();
            for (a, p) in head.args.iter().enumerate() {
                e.text.push_str("let key='key: {\n");
                let expr = e.access_key(p);
                writeln!(
                    e.text,
                    "Some({expr})}}; if core.access_consider(&mut range,{a},key) {{break 'keys;}}"
                )
                .unwrap();
            }
            writeln!(e.text, "}} }} self.ranges[{h}]=Some(range);}}").unwrap();
            writeln!(e.text,"let Some(id)=core.access_next(self.ranges[{h}].as_mut().unwrap()) else {{self.ranges[{h}]=None; {} }};",if h==0 {"return Selection::Done;".into()} else {format!("self.depth={};return Selection::Yield;",h-1)}).unwrap();
            writeln!(e.text,"if self.ids[..{h}].contains(&id) {{return Selection::Yield;}} let Some(args)=core.arguments(id) else {{return Selection::Yield;}};").unwrap();
            for (a, p) in head.args.iter().enumerate() {
                e.pattern(p, &format!("args[{a}]"), "self.frame", "Selection::Yield");
            }
            writeln!(
                e.text,
                "self.ids[{h}]=id; self.depth={}; Selection::Yield }},",
                h + 1
            )
            .unwrap();
            prefix.extend(&head_vars[h]);
        }
        writeln!(e.text,"{hcount}=>{{if !core.eligible({r},&self.ids){{self.depth={};return Selection::Yield;}}",hcount-1).unwrap();
        for Guard::Equal(a, b) in &rule.guards {
            let a = e.term(a, "self.frame");
            let b = e.term(b, "self.frame");
            writeln!(
                e.text,
                "if !core.equal({a},{b}){{self.depth={};return Selection::Yield;}}",
                hcount - 1
            )
            .unwrap();
        }
        let body = e.goal(&rule.body, "self.frame");
        writeln!(e.text,"Selection::Found(Application{{rule:{r},ids:self.ids.to_vec(),body:{body},next:self.frame.next}}) }}, _=>unreachable!(),}} }} }}").unwrap();
        text.push_str(&e.text);
    }
    writeln!(text,"pub fn {name}_code()->Compiled{{Compiled{{source:{:?},selectors:&[],native:Some(&[{}])}}}}",format!("{rules:?}"),factories.join(",")).unwrap();
    Ok(text)
}
