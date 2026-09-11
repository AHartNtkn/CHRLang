//! Disposable experiment: completed queries, keyed in full under one program owner.
use chr_reuse::continuations::{Mode, Prepared};
use chr_syntax::{Answer, Constraint, Query, Rule, Var};
use std::collections::BTreeMap;
type Key = (Vec<Constraint>, Vec<(String, Var)>);
pub struct Caller {
    prepared: Prepared,
    answers: BTreeMap<Key, Vec<Answer>>,
}
impl Caller {
    pub fn new(rules: Vec<Rule>) -> Result<Self, String> {
        Ok(Self {
            prepared: Prepared::new(rules, Mode::Direct)?,
            answers: BTreeMap::new(),
        })
    }
    pub fn run(&mut self, query: Query, reuse: bool, bound: usize) -> Result<Vec<Answer>, String> {
        if !reuse {
            let batch = self.prepared.start(query)?.advance(bound);
            return if batch.exhausted {
                Ok(batch.answers)
            } else {
                Err("caller step bound".into())
            };
        }
        let key = (query.constraints, query.outputs);
        if let Some(answers) = self.answers.get(&key) {
            return Ok(answers.clone());
        }
        let batch = self
            .prepared
            .start(Query {
                constraints: key.0.clone(),
                outputs: key.1.clone(),
            })?
            .advance(bound);
        if !batch.exhausted {
            return Err("caller step bound".into());
        }
        self.answers.insert(key, batch.answers.clone());
        Ok(batch.answers)
    }
}
