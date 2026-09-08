//! Experimental joins over constructor and source-occurrence relations.
//! A View belongs to one consistent, canonical equality interpretation.
pub mod execute;
pub mod store;
use chr_syntax::{Constraint, Term, Var};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Occurrence(pub usize);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Relation {
    Source(String, usize),
    Constructor(String, usize),
}
#[derive(Clone)]
struct Fact {
    values: Vec<Value>,
    occurrence: Option<Occurrence>,
}
#[derive(Default, Clone)]
struct Table {
    rows: Vec<Fact>,
    live: BTreeSet<usize>,
    columns: Vec<BTreeMap<Value, Vec<usize>>>,
}
#[derive(Default, Clone)]
pub struct View {
    tables: BTreeMap<Relation, Table>,
    occurrences: BTreeSet<Occurrence>,
    locations: BTreeMap<Occurrence, (Relation, usize)>,
    incidence: BTreeMap<Value, BTreeSet<(Relation, usize)>>,
}
impl View {
    pub fn constructor(&mut self, name: &str, parent: Value, children: &[Value]) {
        let values = std::iter::once(parent)
            .chain(children.iter().copied())
            .collect();
        self.insert(
            Relation::Constructor(name.into(), children.len()),
            Fact {
                values,
                occurrence: None,
            },
        );
    }
    pub fn occurrence(&mut self, id: Occurrence, name: &str, args: &[Value]) {
        assert!(
            self.occurrences.insert(id),
            "source occurrence identity reused"
        );
        self.insert(
            Relation::Source(name.into(), args.len()),
            Fact {
                values: args.to_vec(),
                occurrence: Some(id),
            },
        );
    }
    fn insert(&mut self, key: Relation, fact: Fact) {
        let table = self.tables.entry(key.clone()).or_default();
        if table.columns.is_empty() {
            table.columns.resize_with(fact.values.len(), BTreeMap::new);
        }
        let index = table.rows.len();
        if let Some(id) = fact.occurrence {
            self.locations.insert(id, (key.clone(), index));
        }
        table.rows.push(fact);
        table.live.insert(index);
        self.attach(&key, index);
    }
    fn attach(&mut self, key: &Relation, index: usize) {
        let table = self.tables.get_mut(key).unwrap();
        for (column, value) in table.columns.iter_mut().zip(&table.rows[index].values) {
            column.entry(*value).or_default().push(index);
            self.incidence
                .entry(*value)
                .or_default()
                .insert((key.clone(), index));
        }
    }
    fn detach(&mut self, key: &Relation, index: usize) {
        let table = self.tables.get_mut(key).unwrap();
        for (column, value) in table.columns.iter_mut().zip(&table.rows[index].values) {
            let bucket = column.get_mut(value).unwrap();
            bucket.retain(|i| *i != index);
            if bucket.is_empty() {
                column.remove(value);
            }
            if let Some(rows) = self.incidence.get_mut(value) {
                rows.remove(&(key.clone(), index));
                if rows.is_empty() {
                    self.incidence.remove(value);
                }
            }
        }
    }
    fn retire(&mut self, id: Occurrence) {
        let (key, index) = self.locations.remove(&id).unwrap();
        self.detach(&key, index);
        self.tables.get_mut(&key).unwrap().live.remove(&index);
    }
    fn replace_value(&mut self, old: Value, new: Value) -> Vec<(Relation, usize)> {
        let incident = self.incidence.get(&old).cloned().unwrap_or_default();
        let mut changed = Vec::new();
        for (key, index) in incident {
            self.detach(&key, index);
            for value in &mut self.tables.get_mut(&key).unwrap().rows[index].values {
                if *value == old {
                    *value = new;
                }
            }
            self.attach(&key, index);
            if matches!(key, Relation::Constructor(..)) {
                changed.push((key, index));
            }
        }
        changed
    }
    fn descriptors(&self, value: Value) -> Vec<(String, Vec<Value>)> {
        self.incidence
            .get(&value)
            .into_iter()
            .flatten()
            .filter_map(|(key, index)| {
                let Relation::Constructor(name, _) = key else {
                    return None;
                };
                let row = &self.tables[key].rows[*index];
                (row.values[0] == value).then(|| (name.clone(), row.values[1..].to_vec()))
            })
            .collect()
    }
    fn peers(&self, name: &str, children: &[Value]) -> Vec<Value> {
        let key = Relation::Constructor(name.into(), children.len());
        let Some(table) = self.tables.get(&key) else {
            return Vec::new();
        };
        let slots = (0..=children.len()).collect::<Vec<_>>();
        let values = std::iter::once(None)
            .chain(children.iter().copied().map(Some))
            .collect::<Vec<_>>();
        table
            .candidates(&slots, &values)
            .into_iter()
            .filter_map(|i| {
                let row = &table.rows[i];
                (row.values[1..] == *children).then_some(row.values[0])
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}
#[derive(Clone)]
struct Atom {
    relation: Relation,
    slots: Vec<usize>,
    head: Option<usize>,
}
pub struct HeadPlan {
    atoms: Vec<Atom>,
    variables: BTreeMap<Var, usize>,
    slots: usize,
    kept: usize,
    heads: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Match {
    pub kept: Vec<Occurrence>,
    pub removed: Vec<Occurrence>,
    pub bindings: BTreeMap<Var, Value>,
}
#[derive(Default, Debug)]
pub struct Evaluation {
    pub matches: Vec<Match>,
    pub first_relation: Option<Relation>,
}
impl HeadPlan {
    pub fn compile(kept: &[Constraint], removed: &[Constraint]) -> Self {
        let mut plan = Self {
            atoms: Vec::new(),
            variables: BTreeMap::new(),
            slots: 0,
            kept: kept.len(),
            heads: kept.len() + removed.len(),
        };
        for (head, constraint) in kept.iter().chain(removed).enumerate() {
            let slots = constraint.args.iter().map(|term| plan.term(term)).collect();
            plan.atoms.push(Atom {
                relation: Relation::Source(constraint.name.clone(), constraint.args.len()),
                slots,
                head: Some(head),
            });
        }
        plan
    }
    fn fresh(&mut self) -> usize {
        let slot = self.slots;
        self.slots += 1;
        slot
    }
    fn term(&mut self, term: &Term) -> usize {
        match term {
            Term::Var(var) => {
                if let Some(slot) = self.variables.get(var) {
                    return *slot;
                }
                let slot = self.fresh();
                self.variables.insert(*var, slot);
                slot
            }
            Term::App(name, children) => {
                let parent = self.fresh();
                let mut slots = vec![parent];
                slots.extend(children.iter().map(|term| self.term(term)));
                self.atoms.push(Atom {
                    relation: Relation::Constructor(name.clone(), children.len()),
                    slots,
                    head: None,
                });
                parent
            }
        }
    }
    pub fn evaluate(&self, view: &View) -> Evaluation {
        let remaining = (0..self.atoms.len()).collect::<Vec<_>>();
        let mut output = BTreeSet::new();
        let mut first_relation = None;
        self.join(
            view,
            &remaining,
            vec![None; self.slots],
            vec![None; self.heads],
            &mut output,
            &mut first_relation,
        );
        Evaluation {
            matches: output.into_iter().collect(),
            first_relation,
        }
    }
    fn join(
        &self,
        view: &View,
        remaining: &[usize],
        values: Vec<Option<Value>>,
        heads: Vec<Option<Occurrence>>,
        output: &mut BTreeSet<Match>,
        first: &mut Option<Relation>,
    ) {
        if remaining.is_empty() {
            let ids = heads.into_iter().map(Option::unwrap).collect::<Vec<_>>();
            output.insert(Match {
                kept: ids[..self.kept].to_vec(),
                removed: ids[self.kept..].to_vec(),
                bindings: self
                    .variables
                    .iter()
                    .map(|(var, slot)| (*var, values[*slot].unwrap()))
                    .collect(),
            });
            return;
        }
        let (position, index) = remaining
            .iter()
            .enumerate()
            .min_by_key(|(_, i)| {
                let atom = &self.atoms[**i];
                view.tables.get(&atom.relation).map_or(0, |table| {
                    table
                        .bucket(&atom.slots, &values)
                        .map_or(table.live.len(), <[usize]>::len)
                })
            })
            .unwrap();
        let atom = &self.atoms[*index];
        if first.is_none() {
            *first = Some(atom.relation.clone());
        }
        let Some(table) = view.tables.get(&atom.relation) else {
            return;
        };
        let candidates = table.candidates(&atom.slots, &values);
        let mut rest = remaining.to_vec();
        rest.remove(position);
        for index in candidates {
            let row = &table.rows[index];
            let mut next = values.clone();
            let mut compatible = true;
            for (slot, value) in atom.slots.iter().zip(&row.values) {
                if next[*slot].is_some_and(|old| old != *value) {
                    compatible = false;
                    break;
                }
                next[*slot] = Some(*value);
            }
            if !compatible {
                continue;
            }
            let mut next_heads = heads.clone();
            if let Some(head) = atom.head {
                let id = row
                    .occurrence
                    .expect("source relation requires resource identity");
                if heads.contains(&Some(id)) {
                    continue;
                }
                next_heads[head] = Some(id);
            }
            self.join(view, &rest, next, next_heads, output, first);
        }
    }
}

impl Table {
    fn candidates(&self, slots: &[usize], values: &[Option<Value>]) -> Vec<usize> {
        self.bucket(slots, values)
            .map_or_else(|| self.live.iter().copied().collect(), <[usize]>::to_vec)
    }
    /// Cheapest already available column bucket; no row scan for cardinality.
    fn bucket(&self, slots: &[usize], values: &[Option<Value>]) -> Option<&[usize]> {
        let mut best: Option<&[usize]> = None;
        for (column, slot) in self.columns.iter().zip(slots) {
            if let Some(value) = values[*slot] {
                let candidate = column.get(&value).map(Vec::as_slice).unwrap_or(&[]);
                if best.is_none_or(|old| candidate.len() < old.len()) {
                    best = Some(candidate);
                }
            }
        }
        best
    }
}
