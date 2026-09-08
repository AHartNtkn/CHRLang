// Generated from a certified finite recursive source relation.
use chr_persistent::{kernel::{Arena, Bindings, Scope, Term}, Stats};
use chr_syntax::{Answer, Query, Term as Source};
pub fn execute(query: Query) -> Result<Option<Answer>, String> {
let [call] = query.constraints.as_slice() else { return Err("requires exactly one call".into()); };
if call.name != "relation\"\\\nλ" || call.args.len() != 3 { return Err("call predicate or arity differs".into()); }
let mut names = std::collections::BTreeSet::new();
if query.outputs.iter().any(|(name,_)| !names.insert(name)) { return Err("duplicate output name".into()); }
let mut control = &call.args[2];
let mut depth = 0;
loop { match control {
Source::App(name,args) if name == "done\"\\\n终" && args.is_empty() => break,
Source::App(name,args) if name == "more\"\\\n枝" && args.len() == 1 => { depth += 1; control = &args[0]; },
_ => return Err("control is not a finite ground base/step spine".into()),
} }
let mut bindings = Bindings::default();
let mut arena = Arena::default();
let mut stats = Stats::default();
let mut next = 0;
let mut query_scope = Scope::new();
let mut args: Vec<_> = call.args.iter().map(|t| arena.instantiate(t, &mut query_scope, &mut next, &mut stats)).collect();
let outputs: Vec<_> = query.outputs.iter().map(|(name,var)| (name.clone(), arena.instantiate(&Source::Var(*var), &mut query_scope, &mut next, &mut stats))).collect();
for remaining in (0..=depth).rev() {
if remaining == 0 {
let slot_0 = args[1];
let slot_1 = args[0];
if !arena.unify(slot_0, slot_1, &mut bindings, &mut stats) { return Ok(None); }
} else {
let slot_0 = args[0];
let slot_1 = { let value = Term::Var(next); next += 1; value };
let term_0 = arena.make("wrap", vec![slot_1], &mut stats);
if !arena.unify(slot_0, term_0, &mut bindings, &mut stats) { return Ok(None); }
let slot_2 = args[1];
let slot_3 = { let Term::Node(node) = args[2] else { unreachable!("certified ground control") }; arena.node(node).args[0] };
args = vec![slot_1,slot_2,slot_3];
}
}
Ok(Some(Answer { outputs: outputs.into_iter().map(|(name,term)| (name, arena.export(term, &bindings, &mut stats))).collect(), residual: vec![] }))
}
