//! Query-owned equation reuse with stable constructor handles and explicit validity.
//! This is an operation kernel, not a source scheduler or a branch-merging rule.
use chr_persistent::{
    Stats,
    kernel::{Arena, Bindings, Term},
};
use chr_syntax::Term as Source;
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

#[derive(Clone)]
pub struct Handle {
    owner: Rc<()>,
    term: Term,
}
#[derive(Clone)]
pub struct Context {
    owner: Rc<()>,
    bindings: Bindings,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Policy {
    Direct,
    Exact,
    Dependencies,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ForeignOwner;
#[derive(Debug)]
pub struct Outcome {
    pub success: bool,
    pub hit: bool,
    pub changed: Vec<u64>,
}
struct Entry {
    policy: Policy,
    input: Option<Bindings>,
    dependencies: Vec<(u64, Option<Term>)>,
    output: Option<Bindings>,
    success: bool,
    delta: Vec<(u64, Term)>,
    changed: Vec<u64>,
}
/// All contexts share this append-only constructor arena. Variables are minted
/// once per session, including after forks; callers cannot forge numeric handles.
pub struct Session {
    owner: Rc<()>,
    arena: Arena,
    next_var: u64,
    cache: HashMap<(Term, Term), Entry>,
    fifo: VecDeque<(Term, Term)>,
    capacity: usize,
    stats: Stats,
}
impl Session {
    pub fn new(capacity: usize) -> Self {
        Self {
            owner: Rc::new(()),
            arena: Arena::default(),
            next_var: 0,
            cache: HashMap::new(),
            fifo: VecDeque::new(),
            capacity,
            stats: Stats::default(),
        }
    }
    pub fn variable(&mut self) -> Handle {
        let id = self.next_var;
        self.next_var = id
            .checked_add(1)
            .expect("session variable identity exhausted");
        Handle {
            owner: self.owner.clone(),
            term: Term::Var(id),
        }
    }
    pub fn make(&mut self, name: &str, args: &[Handle]) -> Result<Handle, ForeignOwner> {
        if args.iter().any(|h| !Rc::ptr_eq(&h.owner, &self.owner)) {
            return Err(ForeignOwner);
        }
        let term = self
            .arena
            .make(name, args.iter().map(|h| h.term).collect(), &mut self.stats);
        Ok(Handle {
            owner: self.owner.clone(),
            term,
        })
    }
    pub fn context(&self) -> Context {
        Context {
            owner: self.owner.clone(),
            bindings: Bindings::default(),
        }
    }
    fn check(&self, h: &Handle, c: &Context) -> Result<(), ForeignOwner> {
        if Rc::ptr_eq(&h.owner, &self.owner) && Rc::ptr_eq(&c.owner, &self.owner) {
            Ok(())
        } else {
            Err(ForeignOwner)
        }
    }
    pub fn export(&mut self, h: &Handle, c: &Context) -> Result<Source, ForeignOwner> {
        self.check(h, c)?;
        Ok(self.arena.export(h.term, &c.bindings, &mut self.stats))
    }
    pub fn entries(&self) -> usize {
        self.cache.len()
    }
    pub fn clear(&mut self) {
        self.cache.clear();
        self.fifo.clear();
    }
    /// Collect the entire reachable input-variable closure only on a miss.
    /// This is conservative: variables irrelevant to an early clash may occur
    /// here. Hits inspect binding dependencies without reconstructing terms.
    fn dependencies(
        &mut self,
        left: Term,
        right: Term,
        bindings: &Bindings,
    ) -> Vec<(u64, Option<Term>)> {
        let mut todo = vec![left, right];
        let mut nodes = HashSet::new();
        let mut variables = BTreeMap::new();
        while let Some(t) = todo.pop() {
            match t {
                Term::Node(id) if nodes.insert(id) => todo.extend(&self.arena.node(id).args),
                Term::Var(v) if !variables.contains_key(&v) => {
                    let binding = bindings.get(&v, &mut self.stats.storage);
                    variables.insert(v, binding);
                    if let Some(term) = binding {
                        todo.push(term);
                    }
                }
                _ => (),
            }
        }
        variables.into_iter().collect()
    }
    pub fn unify(
        &mut self,
        left: &Handle,
        right: &Handle,
        context: &mut Context,
        policy: Policy,
    ) -> Result<Outcome, ForeignOwner> {
        self.check(left, context)?;
        self.check(right, context)?;
        let key = (left.term, right.term);
        if policy != Policy::Direct
            && let Some(e) = self.cache.get(&key)
        {
            let valid = e.policy == policy
                && match policy {
                    Policy::Exact => e
                        .input
                        .as_ref()
                        .expect("exact input")
                        .same_root(&context.bindings),
                    Policy::Dependencies => e
                        .dependencies
                        .iter()
                        .all(|(v, b)| context.bindings.get(v, &mut self.stats.storage) == *b),
                    Policy::Direct => false,
                };
            if valid {
                if e.success {
                    if policy == Policy::Exact {
                        context.bindings = e.output.as_ref().expect("exact output").clone();
                    } else {
                        for (v, t) in &e.delta {
                            context.bindings.insert(*v, *t, &mut self.stats.storage);
                        }
                    }
                }
                return Ok(Outcome {
                    success: e.success,
                    hit: true,
                    changed: e.changed.clone(),
                });
            }
        }
        let input = context.bindings.clone();
        let dependencies = if policy == Policy::Dependencies && self.capacity > 0 {
            self.dependencies(left.term, right.term, &input)
        } else {
            vec![]
        };
        let mut changed = Vec::new();
        let success = self.arena.unify_record(
            left.term,
            right.term,
            &mut context.bindings,
            &mut self.stats,
            &mut changed,
        );
        if policy != Policy::Direct && self.capacity > 0 {
            let delta = changed
                .iter()
                .filter(|_| policy == Policy::Dependencies)
                .map(|v| {
                    (
                        *v,
                        context
                            .bindings
                            .get(v, &mut self.stats.storage)
                            .expect("successful changed binding"),
                    )
                })
                .collect();
            if !self.cache.contains_key(&key) {
                if self.cache.len() == self.capacity {
                    self.cache
                        .remove(&self.fifo.pop_front().expect("full cache order"));
                }
                self.fifo.push_back(key);
            }
            self.cache.insert(
                key,
                Entry {
                    policy,
                    input: (policy == Policy::Exact).then_some(input),
                    dependencies,
                    output: (success && policy == Policy::Exact).then(|| context.bindings.clone()),
                    success,
                    delta,
                    changed: changed.clone(),
                },
            );
        }
        Ok(Outcome {
            success,
            hit: false,
            changed,
        })
    }
}
