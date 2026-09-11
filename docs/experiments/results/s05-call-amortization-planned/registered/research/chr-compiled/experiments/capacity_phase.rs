//! Initial consuming-capacity phase with ordered residual and binding transport.
//! Complete finite alternatives only; callers retain their own execution policy.
use super::*;
pub struct Continuation {
    pub query: Query,
    pub bindings: Vec<(Var, Term)>,
}
pub struct Prepared<const DIAGNOSTICS: bool = false> {
    relation: super::Prepared<DIAGNOSTICS>,
    producer_order: Vec<Key>,
}
impl<const DIAGNOSTICS: bool> Prepared<DIAGNOSTICS> {
    pub fn new(source: &[Rule], prefix: usize) -> Result<Self, Error> {
        let phase = source.get(..prefix).ok_or(Error::Source)?;
        let relation = super::Prepared::new(phase)?;
        let producer_order = phase[..phase.len() - 2]
            .iter()
            .map(|r| key(&r.removed[0]))
            .collect();
        Ok(Self {
            relation,
            producer_order,
        })
    }
    pub fn solve(&self, q: &Query, limits: Limits) -> Result<Vec<Continuation>, Error> {
        let mut names = BTreeSet::new();
        if q.constraints.len() > 10000
            || q.outputs.len() > 10000
            || q.outputs.iter().any(|(name, _)| !names.insert(name))
        {
            return Err(Error::Query);
        }
        let mut vars = q.outputs.iter().map(|(_, v)| *v).collect::<BTreeSet<_>>();
        for c in &q.constraints {
            if self.relation.producers.contains_key(&key(c))
                && let Term::Var(v) = c.args[0]
            {
                vars.insert(v);
            }
        }
        let vars = vars.into_iter().collect::<Vec<_>>();
        let probe = Query {
            constraints: q.constraints.clone(),
            outputs: vars
                .iter()
                .enumerate()
                .map(|(i, v)| (format!("slot{i}"), *v))
                .collect(),
        };
        let report = self
            .relation
            .solve_ordered(&probe, limits, Some(&self.producer_order))?;
        Ok(report
            .answers
            .into_iter()
            .map(|answer| {
                let bindings = vars
                    .iter()
                    .copied()
                    .zip(answer.outputs.into_iter().map(|(_, t)| t))
                    .filter(|(v, t)| *t != Term::Var(*v))
                    .collect();
                Continuation {
                    query: Query {
                        constraints: answer.residual,
                        outputs: q.outputs.clone(),
                    },
                    bindings,
                }
            })
            .collect())
    }
}
