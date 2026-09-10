//! Explicit choices with copied branch owners over the existing graph-scanning mechanism.
use super::*;
pub struct Prepared {
    program: Arc<Program>,
}
impl Prepared {
    pub fn compile(rules: &[Rule]) -> Result<Self, &'static str> {
        if rules
            .iter()
            .any(|r| (r.kept.is_empty() && r.removed.is_empty()) || !r.guards.is_empty())
        {
            return Err("local graph search requires nonempty heads and no guards");
        }
        Ok(Self {
            program: Arc::new(Program {
                rules: rules.to_vec(),
            }),
        })
    }
    pub fn start<const METRICS: bool>(&self, q: &Query) -> Search<METRICS> {
        Search {
            frontier: VecDeque::from([Branch {
                execution: self.program.start_mode(q, false),
                pending: vec![],
                env: Env::new(),
            }]),
        }
    }
    pub fn owners(&self) -> usize {
        Arc::strong_count(&self.program)
    }
}
#[derive(Clone)]
struct Branch<const METRICS: bool> {
    execution: Execution<METRICS>,
    pending: Vec<Goal>,
    env: Env,
}
pub struct Search<const METRICS: bool> {
    frontier: VecDeque<Branch<METRICS>>,
}
pub enum Event {
    Progress,
    Answer(Answer),
    Exhausted,
}
impl<const METRICS: bool> Search<METRICS> {
    /// One goal or source-selection turn. Matching/equality may do size-dependent work.
    pub fn tick(&mut self) -> Event {
        let Some(mut branch) = self.frontier.pop_front() else {
            return Event::Exhausted;
        };
        if let Some(goal) = branch.pending.pop() {
            match goal {
                Goal::And(goals) => branch.pending.extend(goals.into_iter().rev()),
                Goal::Or(left, right) => {
                    let mut other = branch.clone();
                    branch.pending.push(*left);
                    other.pending.push(*right);
                    self.frontier.push_back(other);
                }
                primitive => branch.execution.body(&primitive, &mut branch.env),
            }
            if !branch.execution.graph.failed {
                self.frontier.push_back(branch);
            }
            return Event::Progress;
        }
        let program = branch.execution.program.clone();
        for (i, rule) in program.rules.iter().enumerate() {
            let heads = rule.kept.iter().chain(&rule.removed).collect::<Vec<_>>();
            if let Some((ids, env)) = branch.execution.select(i, &heads, &mut vec![], Env::new()) {
                branch.execution.history.insert((i, ids.clone()));
                for id in &ids[rule.kept.len()..] {
                    branch.execution.live.remove(id);
                }
                if METRICS {
                    branch.execution.firings += 1;
                }
                branch.env = env;
                branch.pending.push(rule.body.clone());
                self.frontier.push_back(branch);
                return Event::Progress;
            }
        }
        match branch.execution.answer() {
            Some(a) => Event::Answer(a),
            None => Event::Progress,
        }
    }
}
