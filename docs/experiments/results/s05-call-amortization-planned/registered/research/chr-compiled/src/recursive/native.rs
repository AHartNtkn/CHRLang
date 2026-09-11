//! Native emission consumes only the authoritative certificate's plans.
use super::{Plan, Prepared, Source, Var};
use std::{collections::BTreeMap, fmt::Write};

struct Emitter<'a> {
    plan: &'a Plan,
    control: usize,
    registers: BTreeMap<u64, String>,
    temporary: usize,
    code: String,
}
impl Emitter<'_> {
    fn term(&mut self, term: &Source) -> String {
        match term {
            Source::Var(Var(id)) => {
                if let Some(register) = self.registers.get(id) {
                    return register.clone();
                }
                let register = format!("slot_{}", self.registers.len());
                let value = if let Some(position) =
                    self.plan.parameters.iter().position(|p| *p == Some(*id))
                {
                    format!("args[{position}]")
                } else if self.plan.child == Some(*id) {
                    format!(
                        "{{ let Term::Node(node) = args[{}] else {{ unreachable!(\"certified ground control\") }}; arena.node(node).args[0] }}",
                        self.control
                    )
                } else {
                    "{ let value = Term::Var(next); next += 1; value }".into()
                };
                writeln!(self.code, "let {register} = {value};").unwrap();
                self.registers.insert(*id, register.clone());
                register
            }
            Source::App(name, children) => {
                let args = children
                    .iter()
                    .map(|t| self.term(t))
                    .collect::<Vec<_>>()
                    .join(",");
                let result = format!("term_{}", self.temporary);
                self.temporary += 1;
                writeln!(
                    self.code,
                    "let {result} = arena.make({name:?}, vec![{args}], &mut stats);"
                )
                .unwrap();
                result
            }
        }
    }
}
fn emit_plan(plan: &Plan, control: usize) -> String {
    let mut emitter = Emitter {
        plan,
        control,
        registers: BTreeMap::new(),
        temporary: 0,
        code: String::new(),
    };
    for (left, right) in &plan.equations {
        let left = emitter.term(left);
        let right = emitter.term(right);
        writeln!(
            emitter.code,
            "if !arena.unify({left}, {right}, &mut bindings, &mut stats) {{ return Ok(None); }}"
        )
        .unwrap();
    }
    if let Some(next) = &plan.next {
        let args = next
            .iter()
            .map(|t| emitter.term(t))
            .collect::<Vec<_>>()
            .join(",");
        writeln!(emitter.code, "args = vec![{args}];").unwrap();
    }
    emitter.code
}
impl Prepared {
    /// Emit a standalone module depending only on `chr-syntax` and `chr-persistent`.
    /// `execute` returns Err for unsupported queries and None for finite failure.
    /// Body variables and constructor operations are native statements; only input
    /// query terms use syntax instantiation. Emission does not widen admission.
    pub fn emit_rust(&self) -> String {
        let mut code = String::from(
            r#"// Generated from a certified finite recursive source relation.
use chr_persistent::{kernel::{Arena, Bindings, Scope, Term}, Stats};
use chr_syntax::{Answer, Query, Term as Source};
pub fn execute(query: Query) -> Result<Option<Answer>, String> {
let [call] = query.constraints.as_slice() else { return Err("requires exactly one call".into()); };
"#,
        );
        writeln!(code,"if call.name != {:?} || call.args.len() != {} {{ return Err(\"call predicate or arity differs\".into()); }}",self.predicate,self.arity).unwrap();
        code.push_str(r#"let mut names = std::collections::BTreeSet::new();
if query.outputs.iter().any(|(name,_)| !names.insert(name)) { return Err("duplicate output name".into()); }
"#);
        writeln!(code, "let mut control = &call.args[{}];", self.control).unwrap();
        code.push_str("let mut depth = 0;\nloop { match control {\n");
        writeln!(
            code,
            "Source::App(name,args) if name == {:?} && args.is_empty() => break,",
            self.base_constructor
        )
        .unwrap();
        writeln!(code,"Source::App(name,args) if name == {:?} && args.len() == 1 => {{ depth += 1; control = &args[0]; }},",self.step_constructor).unwrap();
        code.push_str(
            r#"_ => return Err("control is not a finite ground base/step spine".into()),
} }
"#,
        );
        code.push_str(
            if self.base.equations.is_empty() && self.step.equations.is_empty() {
                "let bindings = Bindings::default();\n"
            } else {
                "let mut bindings = Bindings::default();\n"
            },
        );
        code.push_str(r#"let mut arena = Arena::default();
let mut stats = Stats::default();
let mut next = 0;
let mut query_scope = Scope::new();
let mut args: Vec<_> = call.args.iter().map(|t| arena.instantiate(t, &mut query_scope, &mut next, &mut stats)).collect();
let outputs: Vec<_> = query.outputs.iter().map(|(name,var)| (name.clone(), arena.instantiate(&Source::Var(*var), &mut query_scope, &mut next, &mut stats))).collect();
for remaining in (0..=depth).rev() {
if remaining == 0 {
"#);
        code.push_str(&emit_plan(&self.base, self.control));
        code.push_str("} else {\n");
        code.push_str(&emit_plan(&self.step, self.control));
        code.push_str(r#"}
}
Ok(Some(Answer { outputs: outputs.into_iter().map(|(name,term)| (name, arena.export(term, &bindings, &mut stats))).collect(), residual: vec![] }))
}
"#);
        code
    }
}
