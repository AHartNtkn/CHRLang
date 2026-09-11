//! Source-certified removal of a private traversal's known common depth.
use chr_syntax::{Constraint, Goal, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet};
type Key = (String, usize);
fn key(c: &Constraint) -> Key {
    (c.name.clone(), c.args.len())
}
fn occurrences(t: &Term, v: Var) -> usize {
    match t {
        Term::Var(x) => usize::from(*x == v),
        Term::App(_, xs) => xs.iter().map(|x| occurrences(x, v)).sum(),
    }
}
fn calls(g: &Goal, out: &mut Vec<Key>) {
    match g {
        Goal::Constraint(c) => out.push(key(c)),
        Goal::And(gs) => {
            for g in gs {
                calls(g, out)
            }
        }
        Goal::Or(a, b) => {
            calls(a, out);
            calls(b, out)
        }
        _ => (),
    }
}
fn prefix(mut t: &Term, v: Var, succ: &str) -> bool {
    loop {
        match t {
            Term::Var(x) => return *x == v,
            Term::App(n, xs) if n == succ && xs.len() == 1 => t = &xs[0],
            _ => return false,
        }
    }
}
fn paths(g: &Goal, depth: Var, succ: &str, run: &Key) -> Option<BTreeSet<usize>> {
    match g {
        Goal::Fail => Some(BTreeSet::new()),
        Goal::True => Some(BTreeSet::from([0])),
        Goal::Unify(a, b) => {
            (occurrences(a, depth) + occurrences(b, depth) == 0).then(|| BTreeSet::from([0]))
        }
        Goal::Constraint(c) => {
            if key(c) == *run {
                (prefix(&c.args[0], depth, succ) && occurrences(&c.args[1], depth) == 0)
                    .then(|| BTreeSet::from([1]))
            } else {
                c.args
                    .iter()
                    .all(|x| occurrences(x, depth) == 0)
                    .then(|| BTreeSet::from([0]))
            }
        }
        Goal::Or(a, b) => {
            let mut a = paths(a, depth, succ, run)?;
            a.extend(paths(b, depth, succ, run)?);
            Some(a)
        }
        Goal::And(gs) => {
            let mut counts = BTreeSet::from([0]);
            for g in gs {
                let next = paths(g, depth, succ, run)?;
                let mut sums = BTreeSet::new();
                for a in &counts {
                    for b in &next {
                        let n = a + b;
                        if n > 1 {
                            return None;
                        }
                        sums.insert(n);
                    }
                }
                counts = sums;
            }
            Some(counts)
        }
    }
}
fn matches(p: &Term, t: &Term, env: &mut BTreeMap<Var, Term>) -> bool {
    match (p, t) {
        (Term::Var(v), t) => match env.get(v) {
            Some(old) => old == t,
            None => {
                env.insert(*v, t.clone());
                true
            }
        },
        (Term::App(n, xs), Term::App(m, ys)) => {
            n == m && xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| matches(x, y, env))
        }
        _ => false,
    }
}
pub struct Program {
    entry: Constraint,
    slot: usize,
    run: Key,
    fuel: Key,
    permit: Key,
    succ: String,
    zero: String,
    heads: BTreeSet<Key>,
}
impl Program {
    pub fn infer(rules: &[Rule]) -> Result<Self, &'static str> {
        for (si, step) in rules.iter().enumerate() {
            if step.kept.len() != 1
                || step.removed.len() != 2
                || !step.guards.is_empty()
                || !step.kept[0].args.is_empty()
            {
                continue;
            }
            let Goal::Constraint(body) = &step.body else {
                continue;
            };
            if body.args.len() != 2 {
                continue;
            }
            let (Term::Var(n), Term::Var(r)) = (&body.args[0], &body.args[1]) else {
                continue;
            };
            if n == r {
                continue;
            }
            for pos in 0..2 {
                let head = &step.removed[pos];
                let fuel = &step.removed[1 - pos];
                if key(head) != key(body) || !fuel.args.is_empty() {
                    continue;
                }
                let Term::App(succ, xs) = &head.args[0] else {
                    continue;
                };
                if xs != &vec![Term::Var(*n)] || head.args[1] != Term::Var(*r) {
                    continue;
                }
                let run = key(body);
                let fuel = key(fuel);
                let permit = key(&step.kept[0]);
                if run == fuel || run == permit || fuel == permit {
                    continue;
                }
                let terminals = rules
                    .iter()
                    .enumerate()
                    .filter(|(i, x)| {
                        *i != si
                            && x.kept.is_empty()
                            && x.removed.len() == 1
                            && key(&x.removed[0]) == run
                            && x.guards.is_empty()
                    })
                    .collect::<Vec<_>>();
                if terminals.len() != 1 {
                    continue;
                }
                let (ti, terminal) = terminals[0];
                let [Term::App(zero, zargs), Term::Var(_)] = terminal.removed[0].args.as_slice()
                else {
                    continue;
                };
                if !zargs.is_empty() {
                    continue;
                }
                for (ei, entry) in rules.iter().enumerate() {
                    if ei == si
                        || ei == ti
                        || !entry.kept.is_empty()
                        || entry.removed.len() != 1
                        || !entry.guards.is_empty()
                    {
                        continue;
                    }
                    let e = &entry.removed[0];
                    let ek = key(e);
                    if ek == run || ek == fuel || ek == permit {
                        continue;
                    }
                    for (slot, arg) in e.args.iter().enumerate() {
                        let Term::Var(depth) = arg else {
                            continue;
                        };
                        if e.args.iter().map(|x| occurrences(x, *depth)).sum::<usize>() != 1
                            || paths(&entry.body, *depth, succ, &run) != Some(BTreeSet::from([1]))
                        {
                            continue;
                        }
                        let mut valid = true;
                        let mut heads = BTreeSet::new();
                        for (i, rule) in rules.iter().enumerate() {
                            for h in rule.kept.iter().chain(&rule.removed) {
                                let k = key(h);
                                heads.insert(k.clone());
                                if (k == run && i != si && i != ti)
                                    || (k == fuel && i != si)
                                    || (k == ek && i != ei)
                                {
                                    valid = false;
                                }
                            }
                            if rule.removed.iter().any(|h| key(h) == permit) {
                                valid = false;
                            }
                            let mut cs = vec![];
                            calls(&rule.body, &mut cs);
                            if cs.iter().any(|k| {
                                *k == fuel
                                    || *k == permit
                                    || *k == ek
                                    || (*k == run && i != si && i != ei)
                            }) {
                                valid = false;
                            }
                        }
                        // Entry effects that activate another rule could race the
                        // terminal after shortening. Inert residual posts commute.
                        let mut entry_calls = vec![];
                        calls(&entry.body, &mut entry_calls);
                        if entry_calls.iter().any(|k| *k != run && heads.contains(k)) {
                            valid = false;
                        }
                        if valid {
                            return Ok(Self {
                                entry: e.clone(),
                                slot,
                                run: run.clone(),
                                fuel: fuel.clone(),
                                permit: permit.clone(),
                                succ: succ.clone(),
                                zero: zero.clone(),
                                heads,
                            });
                        }
                    }
                }
            }
        }
        Err("no certified private traversal and entry")
    }
    pub fn lower(&self, q: &Query) -> Result<Query, &'static str> {
        let ek = key(&self.entry);
        let mut entries = vec![];
        let mut fuel = 0;
        let mut permit = false;
        for (i, c) in q.constraints.iter().enumerate() {
            let k = key(c);
            if k == ek {
                entries.push(i);
            } else if k == self.fuel {
                fuel += 1;
            } else if k == self.permit {
                permit = true;
            } else if k == self.run || self.heads.contains(&k) {
                return Err("initial active interference");
            }
        }
        if entries.len() != 1 || !permit {
            return Err("single entry and permit required");
        }
        let index = entries[0];
        let entry = &q.constraints[index];
        let mut env = BTreeMap::new();
        if !self
            .entry
            .args
            .iter()
            .zip(&entry.args)
            .all(|(p, t)| matches(p, t, &mut env))
        {
            return Err("entry head not established");
        }
        let mut t = &entry.args[self.slot];
        let mut depth = 0usize;
        loop {
            match t {
                Term::App(n, xs) if n == &self.zero && xs.is_empty() => break,
                Term::App(n, xs) if n == &self.succ && xs.len() == 1 => {
                    depth = depth.checked_add(1).ok_or("depth overflow")?;
                    t = &xs[0];
                }
                _ => return Err("depth is not a ground traversal"),
            }
        }
        if fuel < depth {
            return Err("insufficient common fuel");
        }
        let mut remaining = depth;
        let mut out = q.clone();
        out.constraints[index].args[self.slot] = Term::App(self.zero.clone(), vec![]);
        out.constraints.retain(|c| {
            if key(c) == self.fuel && remaining > 0 {
                remaining -= 1;
                false
            } else {
                true
            }
        });
        Ok(out)
    }
}
