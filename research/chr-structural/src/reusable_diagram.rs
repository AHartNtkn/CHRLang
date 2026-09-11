//! Experimental finite logical sets. No raw-answer multiplicity or source-effect contract.
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
pub enum Expr {
    Equal(usize, usize),
    Different(usize, usize),
    And(Vec<Expr>),
}
#[derive(Clone)]
struct Node {
    variable: usize,
    children: Vec<usize>,
}
#[derive(Clone)]
pub struct Diagram {
    variables: usize,
    alphabet: usize,
    nodes: Vec<Node>,
    unique: BTreeMap<(usize, Vec<usize>), usize>,
    root: usize,
}
struct Budget(usize);
impl Budget {
    fn tick(&mut self) -> Result<(), String> {
        self.0 = self.0.checked_sub(1).ok_or("diagram work bound")?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    And,
    Or,
    Difference,
}
impl Diagram {
    fn empty(variables: usize, alphabet: usize) -> Self {
        Self {
            variables,
            alphabet,
            nodes: vec![
                Node {
                    variable: variables,
                    children: vec![]
                };
                2
            ],
            unique: BTreeMap::new(),
            root: 0,
        }
    }
    fn node(
        &mut self,
        variable: usize,
        children: Vec<usize>,
        budget: &mut Budget,
    ) -> Result<usize, String> {
        budget.tick()?;
        if children.iter().all(|x| *x == children[0]) {
            return Ok(children[0]);
        }
        let key = (variable, children);
        if let Some(id) = self.unique.get(&key) {
            return Ok(*id);
        }
        let id = self.nodes.len();
        self.nodes.push(Node {
            variable,
            children: key.1.clone(),
        });
        self.unique.insert(key, id);
        Ok(id)
    }
    fn apply(
        &mut self,
        op: Op,
        a: usize,
        b: usize,
        memo: &mut BTreeMap<(Op, usize, usize), usize>,
        budget: &mut Budget,
    ) -> Result<usize, String> {
        budget.tick()?;
        if a < 2 && b < 2 {
            return Ok(usize::from(match op {
                Op::And => a == 1 && b == 1,
                Op::Or => a == 1 || b == 1,
                Op::Difference => a == 1 && b == 0,
            }));
        }
        if let Some(id) = memo.get(&(op, a, b)) {
            return Ok(*id);
        }
        let variable = self.nodes[a].variable.min(self.nodes[b].variable);
        let mut children = Vec::with_capacity(self.alphabet);
        for value in 0..self.alphabet {
            let x = if self.nodes[a].variable == variable {
                self.nodes[a].children[value]
            } else {
                a
            };
            let y = if self.nodes[b].variable == variable {
                self.nodes[b].children[value]
            } else {
                b
            };
            children.push(self.apply(op, x, y, memo, budget)?);
        }
        let id = self.node(variable, children, budget)?;
        memo.insert((op, a, b), id);
        Ok(id)
    }
    fn expression(&mut self, e: &Expr, budget: &mut Budget) -> Result<usize, String> {
        budget.tick()?;
        match e {
            Expr::And(es) => {
                let mut root = 1;
                let mut memo = BTreeMap::new();
                for e in es {
                    let rhs = self.expression(e, budget)?;
                    root = self.apply(Op::And, root, rhs, &mut memo, budget)?;
                }
                Ok(root)
            }
            Expr::Equal(a, b) | Expr::Different(a, b) => {
                if *a >= self.variables || *b >= self.variables {
                    return Err("unknown formula coordinate".into());
                }
                let equal = matches!(e, Expr::Equal(..));
                if a == b {
                    return Ok(usize::from(equal));
                }
                let (a, b) = ((*a).min(*b), (*a).max(*b));
                let mut children = Vec::with_capacity(self.alphabet);
                for x in 0..self.alphabet {
                    let row = (0..self.alphabet)
                        .map(|y| usize::from((x == y) == equal))
                        .collect();
                    children.push(self.node(b, row, budget)?);
                }
                self.node(a, children, budget)
            }
        }
    }
    pub fn compile(
        variables: usize,
        alphabet: usize,
        expression: &Expr,
        limit: usize,
    ) -> Result<Self, String> {
        if alphabet == 0 {
            return Err("alphabet must be nonempty".into());
        }
        let mut d = Self::empty(variables, alphabet);
        d.root = d.expression(expression, &mut Budget(limit))?;
        Ok(d)
    }
    pub fn contains(&self, assignment: &[usize]) -> Result<bool, String> {
        if assignment.len() != self.variables || assignment.iter().any(|x| *x >= self.alphabet) {
            return Err("invalid complete assignment".into());
        }
        let mut id = self.root;
        while id >= 2 {
            let n = &self.nodes[id];
            id = n.children[assignment[n.variable]];
        }
        Ok(id == 1)
    }
    fn import(
        &mut self,
        other: &Self,
        id: usize,
        memo: &mut BTreeMap<usize, usize>,
        budget: &mut Budget,
    ) -> Result<usize, String> {
        budget.tick()?;
        if id < 2 {
            return Ok(id);
        }
        if let Some(x) = memo.get(&id) {
            return Ok(*x);
        }
        let n = &other.nodes[id];
        let mut children = Vec::with_capacity(self.alphabet);
        for c in &n.children {
            children.push(self.import(other, *c, memo, budget)?);
        }
        let out = self.node(n.variable, children, budget)?;
        memo.insert(id, out);
        Ok(out)
    }
    fn combine(&self, other: &Self, op: Op, limit: usize) -> Result<Self, String> {
        if self.variables != other.variables || self.alphabet != other.alphabet {
            return Err("different declared universes".into());
        }
        let mut budget = Budget(limit);
        // Copy only reachable nodes; no cross-arena handles escape.
        let mut out = Self::empty(self.variables, self.alphabet);
        let a = out.import(self, self.root, &mut BTreeMap::new(), &mut budget)?;
        let b = out.import(other, other.root, &mut BTreeMap::new(), &mut budget)?;
        out.root = out.apply(op, a, b, &mut BTreeMap::new(), &mut budget)?;
        Ok(out)
    }
    pub fn union(&self, other: &Self, limit: usize) -> Result<Self, String> {
        self.combine(other, Op::Or, limit)
    }
    pub fn intersection(&self, other: &Self, limit: usize) -> Result<Self, String> {
        self.combine(other, Op::And, limit)
    }
    pub fn included_in(&self, other: &Self, limit: usize) -> Result<bool, String> {
        Ok(self.combine(other, Op::Difference, limit)?.root == 0)
    }
    /// Existentially forget coordinates, retaining the same declared universe.
    pub fn exists(&self, hidden: &[usize], limit: usize) -> Result<Self, String> {
        if hidden.iter().any(|x| *x >= self.variables) {
            return Err("unknown existential coordinate".into());
        }
        fn visit(
            source: &Diagram,
            out: &mut Diagram,
            id: usize,
            hidden: &[usize],
            memo: &mut BTreeMap<usize, usize>,
            apply: &mut BTreeMap<(Op, usize, usize), usize>,
            budget: &mut Budget,
        ) -> Result<usize, String> {
            budget.tick()?;
            if id < 2 {
                return Ok(id);
            }
            if let Some(x) = memo.get(&id) {
                return Ok(*x);
            }
            let n = &source.nodes[id];
            let mut children = Vec::with_capacity(source.alphabet);
            for c in &n.children {
                children.push(visit(source, out, *c, hidden, memo, apply, budget)?);
            }
            let result = if hidden.contains(&n.variable) {
                let mut root = 0;
                for c in children {
                    root = out.apply(Op::Or, root, c, apply, budget)?;
                }
                root
            } else {
                out.node(n.variable, children, budget)?
            };
            memo.insert(id, result);
            Ok(result)
        }
        let mut out = Self::empty(self.variables, self.alphabet);
        out.root = visit(
            self,
            &mut out,
            self.root,
            hidden,
            &mut BTreeMap::new(),
            &mut BTreeMap::new(),
            &mut Budget(limit),
        )?;
        Ok(out)
    }
    /// Visit complete assignments in coordinate order. False means the visitor stopped.
    /// Prefix rejection prunes all extensions; the caller must make that predicate sound.
    /// Callback slices are borrowed; copying/retaining output belongs to the caller.
    pub fn visit_assignments(
        &self,
        limit: usize,
        allow_prefix: impl Fn(&[usize]) -> bool,
        mut emit: impl FnMut(&[usize]) -> bool,
    ) -> Result<bool, String> {
        fn walk(
            d: &Diagram,
            id: usize,
            a: &mut Vec<usize>,
            budget: &mut Budget,
            allow_prefix: &impl Fn(&[usize]) -> bool,
            emit: &mut impl FnMut(&[usize]) -> bool,
        ) -> Result<bool, String> {
            budget.tick()?;
            if id == 0 || !allow_prefix(a) {
                return Ok(true);
            }
            if a.len() == d.variables {
                return Ok(emit(a));
            }
            let variable = a.len();
            for value in 0..d.alphabet {
                let child = if id >= 2 && d.nodes[id].variable == variable {
                    d.nodes[id].children[value]
                } else {
                    id
                };
                a.push(value);
                let complete = walk(d, child, a, budget, allow_prefix, emit)?;
                a.pop();
                if !complete {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        walk(
            self,
            self.root,
            &mut Vec::with_capacity(self.variables),
            &mut Budget(limit),
            &allow_prefix,
            &mut emit,
        )
    }
    /// All allocated nodes, including intermediate nodes; includes both terminals.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
