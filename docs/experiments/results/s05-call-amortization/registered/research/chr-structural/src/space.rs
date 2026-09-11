//! Bounded finite skeletons with shared holes and regular restrictions. Not a general ECTA compiler.
use crate::regular::{self, Automaton};
use chr_syntax::Term;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
#[derive(Clone, Debug)]
pub enum Pattern {
    Hole(usize),
    App(String, Vec<Pattern>),
}
#[derive(Clone, Debug)]
pub struct Request {
    pub pattern: Pattern,
    pub domains: Vec<Vec<Automaton>>,
    pub equalities: Vec<(usize, usize)>,
}
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    FilterFirst,
    FilterSmallest,
    Intersect,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub pattern_nodes: u64,
    pub unions: u64,
    pub domain_compares: u64,
    pub enumeration_cache_hits: u64,
    pub domain_cache_hits: u64,
    pub filter_candidates: u64,
    pub output_nodes: u64,
    pub emitted: u64,
    pub regular: regular::Stats,
}
pub struct Batch {
    pub terms: Vec<Term>,
    pub exhausted: bool,
}
pub struct Space {
    pattern: Pattern,
    mapping: Vec<usize>,
    choices: Vec<Rc<Vec<Term>>>,
    cursor: Vec<usize>,
    done: bool,
    stats: Stats,
}
fn root(parents: &mut [usize], i: usize) -> usize {
    if parents[i] != i {
        parents[i] = root(parents, parents[i]);
    }
    parents[i]
}
fn enum_cached(
    domain: &Automaton,
    max_nodes: usize,
    cache: &mut Vec<(Automaton, Rc<Vec<Term>>)>,
    stats: &mut Stats,
) -> Rc<Vec<Term>> {
    for (other, values) in cache.iter() {
        stats.domain_compares += 1;
        if domain == other {
            stats.enumeration_cache_hits += 1;
            return values.clone();
        }
    }
    let values = Rc::new(domain.enumerate(max_nodes, &mut stats.regular));
    cache.push((domain.clone(), values.clone()));
    values
}
fn term_nodes(t: &Term) -> u64 {
    match t {
        Term::Var(_) => 1,
        Term::App(_, args) => 1 + args.iter().map(term_nodes).sum::<u64>(),
    }
}
impl Space {
    pub fn compile(request: Request, max_nodes: usize, mode: Mode) -> Result<Self, String> {
        fn holes(p: &Pattern, vars: &mut BTreeSet<usize>, stats: &mut Stats) {
            stats.pattern_nodes += 1;
            match p {
                Pattern::Hole(id) => {
                    vars.insert(*id);
                }
                Pattern::App(_, args) => {
                    for p in args {
                        holes(p, vars, stats)
                    }
                }
            }
        }
        let mut stats = Stats::default();
        let mut used = BTreeSet::new();
        holes(&request.pattern, &mut used, &mut stats);
        if used.iter().any(|v| *v >= request.domains.len())
            || request
                .equalities
                .iter()
                .any(|(a, b)| *a >= request.domains.len() || *b >= request.domains.len())
        {
            return Err("hole outside the declared interface".into());
        }
        if request.domains.iter().any(Vec::is_empty) {
            return Err("every hole needs a declared regular domain".into());
        }
        let mut parents = (0..request.domains.len()).collect::<Vec<_>>();
        for (a, b) in request.equalities {
            stats.unions += 1;
            let a = root(&mut parents, a);
            let b = root(&mut parents, b);
            parents[a.max(b)] = a.min(b);
        }
        let used = used
            .into_iter()
            .map(|v| root(&mut parents, v))
            .collect::<BTreeSet<_>>();
        if (0..parents.len()).any(|v| !used.contains(&root(&mut parents, v))) {
            return Err("unreferenced existential class is outside this skeleton interface".into());
        }
        let mut groups: BTreeMap<usize, Vec<Automaton>> = BTreeMap::new();
        for (v, domains) in request.domains.into_iter().enumerate() {
            let group = groups.entry(root(&mut parents, v)).or_default();
            for domain in domains {
                if !group.iter().any(|other| {
                    stats.domain_compares += 1;
                    *other == domain
                }) {
                    group.push(domain);
                }
            }
        }
        let class_ids = groups
            .keys()
            .enumerate()
            .map(|(i, id)| (*id, i))
            .collect::<BTreeMap<_, _>>();
        let mapping = (0..parents.len())
            .map(|v| class_ids[&root(&mut parents, v)])
            .collect();
        let mut enumeration_cache = vec![];
        let mut domain_cache: Vec<(Vec<Automaton>, Rc<Vec<Term>>)> = vec![];
        let mut choices = vec![];
        for domains in groups.into_values() {
            if let Some((_, values)) = domain_cache.iter().find(|(other, _)| {
                stats.domain_compares += 1;
                *other == domains
            }) {
                stats.domain_cache_hits += 1;
                choices.push(values.clone());
                continue;
            }
            let values = match mode {
                Mode::Intersect => {
                    let mut domain = domains[0].clone();
                    for next in domains.iter().skip(1) {
                        domain = domain.intersect(next, &mut stats.regular);
                    }
                    if domain.witness(&mut stats.regular).is_none() {
                        Rc::new(vec![])
                    } else {
                        enum_cached(&domain, max_nodes, &mut enumeration_cache, &mut stats)
                    }
                }
                Mode::FilterFirst | Mode::FilterSmallest => {
                    let (base, candidates) = if matches!(mode, Mode::FilterSmallest) {
                        domains
                            .iter()
                            .enumerate()
                            .map(|(i, d)| {
                                (
                                    i,
                                    enum_cached(d, max_nodes, &mut enumeration_cache, &mut stats),
                                )
                            })
                            .min_by_key(|(_, v)| v.len())
                            .unwrap()
                    } else {
                        (
                            0,
                            enum_cached(&domains[0], max_nodes, &mut enumeration_cache, &mut stats),
                        )
                    };
                    if domains.len() == 1 {
                        candidates
                    } else {
                        Rc::new(
                            candidates
                                .iter()
                                .filter(|t| {
                                    stats.filter_candidates += 1;
                                    domains.iter().enumerate().filter(|(i, _)| *i != base).all(
                                        |(_, d)| {
                                            d.contains_term(t, &mut stats.regular)
                                                .expect("regular enumeration must be ground")
                                        },
                                    )
                                })
                                .cloned()
                                .collect(),
                        )
                    }
                }
            };
            domain_cache.push((domains, values.clone()));
            choices.push(values);
        }
        let done = choices.iter().any(|c| c.is_empty());
        let cursor = vec![0; choices.len()];
        Ok(Self {
            pattern: request.pattern,
            mapping,
            choices,
            cursor,
            done,
            stats,
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn classes(&self) -> usize {
        self.choices.len()
    }
    pub fn retained_candidates(&self) -> usize {
        let mut seen = BTreeSet::new();
        self.choices
            .iter()
            .filter(|c| seen.insert(Rc::as_ptr(c) as usize))
            .map(|c| c.len())
            .sum()
    }
    pub fn solution_count(&self) -> Option<u128> {
        if self.choices.iter().any(|c| c.is_empty()) {
            Some(0)
        } else {
            self.choices
                .iter()
                .try_fold(1u128, |n, c| n.checked_mul(c.len() as u128))
        }
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        fn instantiate(
            pattern: &Pattern,
            mapping: &[usize],
            choices: &[Rc<Vec<Term>>],
            cursor: &[usize],
            stats: &mut Stats,
        ) -> Term {
            match pattern {
                Pattern::Hole(v) => {
                    let value = &choices[mapping[*v]][cursor[mapping[*v]]];
                    stats.output_nodes += term_nodes(value);
                    value.clone()
                }
                Pattern::App(n, args) => {
                    stats.output_nodes += 1;
                    Term::App(
                        n.clone(),
                        args.iter()
                            .map(|p| instantiate(p, mapping, choices, cursor, stats))
                            .collect(),
                    )
                }
            }
        }
        let mut terms = vec![];
        for _ in 0..budget {
            if self.done {
                break;
            }
            terms.push(instantiate(
                &self.pattern,
                &self.mapping,
                &self.choices,
                &self.cursor,
                &mut self.stats,
            ));
            self.stats.emitted += 1;
            let mut advanced = false;
            for i in (0..self.cursor.len()).rev() {
                self.cursor[i] += 1;
                if self.cursor[i] < self.choices[i].len() {
                    advanced = true;
                    break;
                }
                self.cursor[i] = 0;
            }
            if !advanced {
                self.done = true;
            }
        }
        Batch {
            terms,
            exhausted: self.done,
        }
    }
}
