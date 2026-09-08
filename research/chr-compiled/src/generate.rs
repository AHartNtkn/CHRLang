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
        Goal::Or(..) => return Err("R01 does not compile OR".into()),
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
    fn pattern(&mut self, p: &Term, value: &str, frame: &str) {
        match p {
            Term::Var(Var(v)) => {
                writeln!(
                    self.text,
                    "if !core.bind(&mut {frame}, {}, {value}) {{return Selection::Yield;}}",
                    self.slots[v]
                )
                .unwrap();
            }
            Term::App(name, args) => {
                let temp = self.name();
                writeln!(self.text,"let Some({temp})=core.constructor({value},{name:?},{}) else {{return Selection::Yield;}};",args.len()).unwrap();
                for (i, p) in args.iter().enumerate() {
                    self.pattern(p, &format!("{temp}[{i}]"), frame)
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
            Goal::Or(..) => unreachable!("validated"),
        };
        writeln!(self.text, "let {temp}={expr};").unwrap();
        temp
    }
}
/// The result defines `{name}_code() -> Compiled` and native rule selectors.
/// Include inside a module importing Core, Cursor, Candidate, Selection, Application, Work and Compiled.
pub fn emit(name: &str, rules: &[Rule]) -> Result<String, String> {
    if name.is_empty()
        || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
        || name.as_bytes()[0].is_ascii_digit()
    {
        return Err("invalid Rust prefix".into());
    }
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
        writeln!(e.text,"fn {func}(core:&mut Core,cursor:&mut Cursor,anchor:Option<(usize,u64)>)->Selection{{\nmatch cursor.depth {{").unwrap();
        for (h, head) in heads.iter().enumerate() {
            writeln!(e.text,"{h} => {{\nlet id=match core.candidate({r},{h},anchor,cursor){{Candidate::Value(id)=>id,Candidate::Yield=>return Selection::Yield,Candidate::Done=>return Selection::Done}};\nlet Some(args)=core.arguments(id) else{{return Selection::Yield}};\nlet mut frame=core.copy_frame(&cursor.frames[{h}]);").unwrap();
            for (a, p) in head.args.iter().enumerate() {
                e.pattern(p, &format!("args[{a}]"), "frame")
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
        "pub fn {name}_code()->Compiled{{Compiled{{source:{:?},selectors:&[{}]}}}}",
        format!("{rules:?}"),
        funcs.join(",")
    )
    .unwrap();
    Ok(text)
}
