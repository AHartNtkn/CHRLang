//! Resumable ordered traversal of support-compatible occurrence prefixes.
use crate::resources::Resources;
use crate::support::{Arena, Job, Operation, Status, Support};

pub(crate) enum Event {
    Progress,
    Tuple(Vec<u64>),
    Done,
}
pub(crate) struct Cursor {
    ids: Vec<u64>,
    regions: Vec<Support>,
    next: Vec<usize>,
    wait: Option<(u64, Job)>,
}
impl Cursor {
    pub(crate) fn new(anchor: Support) -> Self {
        Self {
            ids: vec![],
            regions: vec![anchor],
            next: vec![0],
            wait: None,
        }
    }
    fn extend(&mut self, id: u64, region: Support) {
        self.ids.push(id);
        self.regions.push(region);
        self.next.push(0);
    }
    pub(crate) fn tick(
        &mut self,
        pools: &[Vec<u64>],
        resources: &Resources,
        arena: &mut Arena,
    ) -> Event {
        if let Some((id, mut job)) = self.wait.take() {
            match job.tick(arena) {
                Status::Pending => self.wait = Some((id, job)),
                Status::Complete(region) if region != Support::FALSE => self.extend(id, region),
                Status::Complete(_) => (),
            }
            return Event::Progress;
        }
        let depth = self.ids.len();
        if depth == pools.len() {
            let ids = self.ids.clone();
            self.ids.pop();
            self.regions.pop();
            self.next.pop();
            return Event::Tuple(ids);
        }
        if let Some(&id) = pools[depth].get(self.next[depth]) {
            self.next[depth] += 1;
            if self.ids.contains(&id) {
                return Event::Progress;
            }
            let live = resources.occurrence(id).unwrap().live;
            let region = self.regions[depth];
            if live == Support::FALSE {
                return Event::Progress;
            }
            if live == Support::TRUE || live == region {
                self.extend(id, region);
            } else if region == Support::TRUE {
                self.extend(id, live);
            } else {
                self.wait = Some((id, arena.job(Operation::And(region, live))));
            }
            Event::Progress
        } else if depth == 0 {
            Event::Done
        } else {
            self.ids.pop();
            self.regions.pop();
            self.next.pop();
            Event::Progress
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordered_distinct_tuples_agree_with_independent_truth_assignments() {
        let mut arena = Arena::new();
        let (_, a) = arena.fresh_variable();
        let (_, b) = arena.fresh_variable();
        let not_a = arena.mk(0, Support::TRUE, Support::FALSE);
        let supports = [a, not_a, b, Support::TRUE];
        let store = crate::equality::Store::new();
        let prepared = crate::engine::PreparedRuleset::new(vec![]).unwrap();
        let mut resources = Resources::new(&prepared, &store);
        for support in supports {
            resources.insert("p", vec![], support, &store).unwrap();
        }
        let pools = vec![vec![0, 1, 2, 3]; 3];
        for (anchor_index, anchor) in [Support::TRUE, a, b].into_iter().enumerate() {
            let mut expected = vec![];
            for x in 0..4 {
                for y in 0..4 {
                    for z in 0..4 {
                        if x == y || x == z || y == z {
                            continue;
                        }
                        if (0..4).any(|bits| {
                            let a = bits & 1 != 0;
                            let b = bits & 2 != 0;
                            let truth = [a, !a, b, true];
                            [true, a, b][anchor_index] && [x, y, z].iter().all(|&id| truth[id])
                        }) {
                            expected.push(vec![x as u64, y as u64, z as u64]);
                        }
                    }
                }
            }
            let mut cursor = Cursor::new(anchor);
            let mut actual = vec![];
            let mut done = false;
            for _ in 0..10_000 {
                match cursor.tick(&pools, &resources, &mut arena) {
                    Event::Tuple(ids) => actual.push(ids),
                    Event::Done => {
                        done = true;
                        break;
                    }
                    Event::Progress => (),
                }
            }
            assert!(done);
            assert_eq!(actual, expected);
        }
    }
}
