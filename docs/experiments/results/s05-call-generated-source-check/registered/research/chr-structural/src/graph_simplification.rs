//! Exact finite-name graph reductions; unresolved hidden cores remain constraints.
use std::collections::{BTreeSet, VecDeque};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    pub left: usize,
    pub right: usize,
    pub equal: bool,
}
pub struct Reduced {
    alphabet: usize,
    visible: Vec<usize>,
    positions: Vec<Option<usize>>,
    branches: Vec<Vec<Relation>>,
    closed: bool,
}
struct Budget(usize);
impl Budget {
    fn tick(&mut self) -> Result<(), String> {
        self.0 = self.0.checked_sub(1).ok_or("graph reduction work bound")?;
        Ok(())
    }
}
impl Reduced {
    pub fn compile(
        variables: usize,
        alphabet: usize,
        branches: &[Vec<(usize, usize)>],
        visible: &[usize],
        limit: usize,
    ) -> Result<Self, String> {
        let mut budget = Budget(limit);
        budget.tick()?;
        if alphabet == 0 {
            return Err("alphabet must be nonempty".into());
        }
        if variables > limit {
            return Err("graph reduction work bound".into());
        }
        let mut positions = vec![None; variables];
        for (i, x) in visible.iter().enumerate() {
            budget.tick()?;
            let slot = positions.get_mut(*x).ok_or("unknown visible coordinate")?;
            if slot.replace(i).is_some() {
                return Err("duplicate visible coordinate".into());
            }
        }
        // Validate the whole source before any contradiction/universal shortcut.
        for branch in branches {
            for (x, y) in branch {
                budget.tick()?;
                if *x >= variables || *y >= variables {
                    return Err("unknown graph coordinate".into());
                }
            }
        }
        let mut result = Vec::new();
        for branch in branches {
            budget.tick()?;
            if branch.iter().any(|(x, y)| x == y) {
                continue;
            }
            let mut graph = vec![BTreeSet::new(); variables];
            for (x, y) in branch {
                budget.tick()?;
                graph[*x].insert(*y);
                graph[*y].insert(*x);
            }
            let reduced = if alphabet == 1 {
                branch.is_empty().then(Vec::new)
            } else if alphabet == 2 {
                parity(&graph, &positions, &mut budget)?
            } else {
                eliminate(&mut graph, &positions, alphabet, &mut budget)?
            };
            if let Some(reduced) = reduced {
                if reduced.is_empty() {
                    result = vec![vec![]];
                    break;
                }
                if !result.contains(&reduced) {
                    result.push(reduced);
                }
            }
        }
        let closed = result
            .iter()
            .flatten()
            .all(|r| positions[r.left].is_some() && positions[r.right].is_some());
        Ok(Self {
            alphabet,
            visible: visible.to_vec(),
            positions,
            branches: result,
            closed,
        })
    }
    pub fn constraints(&self) -> &[Vec<Relation>] {
        &self.branches
    }
    pub fn is_closed(&self) -> bool {
        self.closed
    }
    /// Values follow the declared visible-coordinate order. No allocation per observation.
    pub fn contains(&self, values: &[usize]) -> Result<bool, String> {
        if values.len() != self.visible.len() || values.iter().any(|x| *x >= self.alphabet) {
            return Err("invalid visible assignment".into());
        }
        if !self.closed {
            return Err("unresolved hidden graph core".into());
        }
        Ok(self.branches.iter().any(|b| {
            b.iter().all(|r| {
                (values[self.positions[r.left].unwrap()]
                    == values[self.positions[r.right].unwrap()])
                    == r.equal
            })
        }))
    }
}
fn parity(
    graph: &[BTreeSet<usize>],
    visible: &[Option<usize>],
    budget: &mut Budget,
) -> Result<Option<Vec<Relation>>, String> {
    let mut colors = vec![None; graph.len()];
    let mut constraints = Vec::new();
    for start in 0..graph.len() {
        budget.tick()?;
        if colors[start].is_some() {
            continue;
        }
        colors[start] = Some(false);
        let mut queue = VecDeque::from([start]);
        let mut anchor = None;
        while let Some(x) = queue.pop_front() {
            budget.tick()?;
            if visible[x].is_some() {
                if let Some(a) = anchor {
                    constraints.push(Relation {
                        left: a,
                        right: x,
                        equal: colors[a] == colors[x],
                    });
                } else {
                    anchor = Some(x);
                }
            }
            for y in &graph[x] {
                budget.tick()?;
                let expected = !colors[x].unwrap();
                match colors[*y] {
                    Some(c) if c != expected => return Ok(None),
                    Some(_) => {}
                    None => {
                        colors[*y] = Some(expected);
                        queue.push_back(*y);
                    }
                }
            }
        }
    }
    Ok(Some(constraints))
}
fn eliminate(
    graph: &mut [BTreeSet<usize>],
    visible: &[Option<usize>],
    alphabet: usize,
    budget: &mut Budget,
) -> Result<Option<Vec<Relation>>, String> {
    loop {
        let mut removed = false;
        for x in 0..graph.len() {
            budget.tick()?;
            if visible[x].is_none() && !graph[x].is_empty() && graph[x].len() < alphabet {
                let neighbors = std::mem::take(&mut graph[x]);
                for y in neighbors {
                    budget.tick()?;
                    graph[y].remove(&x);
                }
                removed = true;
            }
        }
        if !removed {
            break;
        }
    }
    let mut seen = vec![false; graph.len()];
    for start in 0..graph.len() {
        budget.tick()?;
        if seen[start] {
            continue;
        }
        let mut queue = VecDeque::from([start]);
        seen[start] = true;
        let mut vertices = 0usize;
        let mut degrees = 0usize;
        while let Some(x) = queue.pop_front() {
            budget.tick()?;
            vertices += 1;
            degrees += graph[x].len();
            for y in &graph[x] {
                budget.tick()?;
                if !seen[*y] {
                    seen[*y] = true;
                    queue.push_back(*y);
                }
            }
        }
        if vertices > alphabet && degrees == vertices * (vertices - 1) {
            return Ok(None);
        }
    }
    let mut constraints = Vec::new();
    for (x, neighbors) in graph.iter().enumerate() {
        for y in neighbors {
            budget.tick()?;
            if x < *y {
                constraints.push(Relation {
                    left: x,
                    right: *y,
                    equal: false,
                });
            }
        }
    }
    Ok(Some(constraints))
}
