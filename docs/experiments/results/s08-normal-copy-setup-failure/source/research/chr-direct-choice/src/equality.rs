//! Contextual finite-tree redirects. Operations are synchronous; no service bound
//! is claimed. Bindings in failed regions may remain retained until graph disposal.
use crate::{Context, Graph, Node, NodeId};

/// Add a cube to a disjoint union without multiplying overlapping alternatives.
fn union(regions: &mut Vec<Context>, added: Context) {
    let mut novel = vec![added];
    for old in regions.iter() {
        novel = novel.iter().flat_map(|r| r.subtract(old)).collect();
    }
    regions.extend(novel);
}
impl Graph {
    pub(crate) fn live(&self, context: &Context) -> Vec<Context> {
        let mut regions = vec![context.clone()];
        for failed in &self.failed {
            regions = regions.iter().flat_map(|r| r.subtract(failed)).collect();
        }
        regions
    }
    /// Follow logical redirects, leaving choice and constructor nodes opaque.
    pub(crate) fn resolve(&self, node: NodeId, context: &Context) -> Vec<(Context, NodeId)> {
        let Node::Unknown(var) = self.nodes[node.0] else {
            return vec![(context.clone(), node)];
        };
        let Some(bindings) = self.bindings.get(&var) else {
            return vec![(context.clone(), node)];
        };
        let mut unbound = vec![context.clone()];
        let mut result = Vec::new();
        for (bound, target) in bindings {
            if let Some(region) = context.intersection(bound) {
                result.extend(self.resolve(*target, &region));
            }
            unbound = unbound.iter().flat_map(|r| r.subtract(bound)).collect();
        }
        result.extend(unbound.into_iter().map(|r| (r, node)));
        result
    }
    fn arms(&self, node: NodeId, context: &Context) -> Vec<(Context, NodeId)> {
        let Node::Choice(label, left, right) = self.nodes[node.0] else {
            return vec![(context.clone(), node)];
        };
        let Some(active) = context.intersection(&self.births[label.0]) else {
            return Vec::new();
        };
        [(false, left), (true, right)]
            .into_iter()
            .filter_map(|(arm, child)| active.select(label, arm).map(|r| (r, child)))
            .collect()
    }
    /// Exact regions in which a currently unbound variable occurs in the value.
    fn occurs(&self, var: u64, node: NodeId, context: &Context) -> Vec<Context> {
        let mut result = Vec::new();
        for (region, target) in self.resolve(node, context) {
            let found = match &self.nodes[target.0] {
                Node::Unknown(id) => {
                    if *id == var {
                        vec![region]
                    } else {
                        Vec::new()
                    }
                }
                Node::App(_, children) => children
                    .iter()
                    .flat_map(|child| self.occurs(var, *child, &region))
                    .collect(),
                Node::Choice(..) => self
                    .arms(target, &region)
                    .into_iter()
                    .flat_map(|(r, n)| self.occurs(var, n, &r))
                    .collect(),
            };
            for r in found {
                union(&mut result, r);
            }
        }
        result
    }
    pub fn fail(&mut self, context: &Context) {
        union(&mut self.failed, context.clone());
    }
    /// Commit a finite-tree equality. A contradiction permanently fails its region.
    /// Returned regions are successful and disjoint, but need not be maximal cubes.
    pub fn unify(&mut self, left: NodeId, right: NodeId, context: &Context) -> Vec<Context> {
        let mut result = Vec::new();
        for region in self.live(context) {
            result.extend(self.equate(left, right, &region));
        }
        result
    }
    fn bind(
        &mut self,
        variable: NodeId,
        var: u64,
        target: NodeId,
        region: &Context,
    ) -> Vec<Context> {
        let occurs = self.occurs(var, target, region);
        // A top-level X alternative is reflexive; demand that choice before
        // distinguishing it from a real constructor cycle.
        if !occurs.is_empty() && matches!(self.nodes[target.0], Node::Choice(..)) {
            let mut result = Vec::new();
            for (r, child) in self.arms(target, region) {
                result.extend(self.equate(variable, child, &r));
            }
            return result;
        }
        for cycle in occurs {
            self.fail(&cycle);
        }
        let regions = self.live(region);
        for r in &regions {
            self.bindings
                .entry(var)
                .or_default()
                .push((r.clone(), target));
        }
        regions
    }
    fn equate(&mut self, left: NodeId, right: NodeId, context: &Context) -> Vec<Context> {
        let mut result = Vec::new();
        for (region, left) in self.resolve(left, context) {
            for (region, right) in self.resolve(right, &region) {
                if left == right {
                    result.push(region);
                    continue;
                }
                match (&self.nodes[left.0], &self.nodes[right.0]) {
                    (Node::Unknown(x), Node::Unknown(y)) if x == y => result.push(region),
                    (Node::Unknown(x), _) => {
                        let x = *x;
                        result.extend(self.bind(left, x, right, &region));
                    }
                    (_, Node::Unknown(y)) => {
                        let y = *y;
                        result.extend(self.bind(right, y, left, &region));
                    }
                    (Node::Choice(..), _) => {
                        for (r, child) in self.arms(left, &region) {
                            result.extend(self.equate(child, right, &r));
                        }
                    }
                    (_, Node::Choice(..)) => {
                        for (r, child) in self.arms(right, &region) {
                            result.extend(self.equate(left, child, &r));
                        }
                    }
                    (Node::App(a, xs), Node::App(b, ys)) if a == b && xs.len() == ys.len() => {
                        let pairs: Vec<_> = xs.iter().copied().zip(ys.iter().copied()).collect();
                        let mut partial = vec![region];
                        for (x, y) in pairs {
                            partial = partial
                                .into_iter()
                                .flat_map(|r| self.equate(x, y, &r))
                                .collect();
                        }
                        result.extend(partial);
                    }
                    _ => self.fail(&region),
                }
            }
        }
        result
    }
    /// Pure finite-tree entailment. This neither instantiates nor fails regions.
    pub fn equal(&self, left: NodeId, right: NodeId, context: &Context) -> Vec<Context> {
        self.live(context)
            .into_iter()
            .flat_map(|r| self.entails(left, right, &r))
            .collect()
    }
    fn entails(&self, left: NodeId, right: NodeId, context: &Context) -> Vec<Context> {
        let mut result = Vec::new();
        for (region, left) in self.resolve(left, context) {
            for (region, right) in self.resolve(right, &region) {
                if left == right {
                    result.push(region);
                    continue;
                }
                match (&self.nodes[left.0], &self.nodes[right.0]) {
                    (Node::Unknown(x), Node::Unknown(y)) if x == y => result.push(region),
                    (Node::Choice(..), _) => {
                        for (r, child) in self.arms(left, &region) {
                            result.extend(self.entails(child, right, &r));
                        }
                    }
                    (_, Node::Choice(..)) => {
                        for (r, child) in self.arms(right, &region) {
                            result.extend(self.entails(left, child, &r));
                        }
                    }
                    (Node::App(a, xs), Node::App(b, ys)) if a == b && xs.len() == ys.len() => {
                        let mut partial = vec![region];
                        for (x, y) in xs.iter().zip(ys) {
                            partial = partial
                                .into_iter()
                                .flat_map(|r| self.entails(*x, *y, &r))
                                .collect();
                        }
                        result.extend(partial);
                    }
                    _ => {}
                }
            }
        }
        result
    }
}
