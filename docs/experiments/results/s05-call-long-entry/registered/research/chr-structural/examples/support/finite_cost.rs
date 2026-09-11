use chr_structural::finite::{Answer, Grammar, Request, Search, Transition};
use chr_syntax::{Term, atom, t};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
pub struct Schema {
    pub width: usize,
    pub family: &'static str,
}
pub const FAMILIES: [&str; 6] = ["all", "equal", "selective", "empty", "redundant", "overlap"];
pub const MODES: [&str; 5] = ["lazy", "reduced", "enumerate", "enum-reuse", "lowered"];
pub struct Prepared {
    grammar: Option<Grammar>,
    pub schema: Schema,
    mode: String,
    source: usize,
    filters: [usize; 2],
    cache: RefCell<BTreeMap<usize, Rc<Vec<Term>>>>,
}
pub enum Running<'g> {
    Lazy(Search<'g>),
    Enumerate {
        grammar: &'g Grammar,
        request: Request,
        values: Rc<Vec<Term>>,
        cursor: usize,
    },
    Lowered {
        width: usize,
        groups: Vec<usize>,
        choices: Vec<Vec<bool>>,
        cursor: Vec<usize>,
        done: bool,
    },
}
pub enum Event {
    Progress,
    Answer(Answer),
    Done,
}
impl Schema {
    pub fn program(self) -> (Grammar, usize, [usize; 2]) {
        assert!(FAMILIES.contains(&self.family));
        let mut states = vec![
            vec![Transition::new("a", []), Transition::new("b", [])],
            vec![Transition::new("nil", [])],
        ];
        let mut source = 1;
        for _ in 0..self.width {
            let id = states.len();
            states.push(vec![Transition::new("cons", [0, source])]);
            source = id;
        }
        if matches!(self.family, "all" | "equal") {
            return (Grammar::new(states).unwrap(), source, [source; 2]);
        }
        let mut filters = [source; 2];
        for (parity, filter) in filters.iter_mut().enumerate() {
            let leaf = states.len();
            let symbol = if parity == 0 { "a" } else { "b" };
            states.push(vec![Transition::new(symbol, [])]);
            let forbidden = states.len();
            states.push(vec![Transition::new(
                if parity == 0 { "c" } else { "d" },
                [],
            )]);
            let duplicate = states.len();
            states.push(vec![
                Transition::new(symbol, []),
                Transition::new(symbol, []),
            ]);
            let mut tail = 1;
            for position in (0..self.width).rev() {
                let head = match self.family {
                    "selective" if position + 1 < self.width => leaf,
                    "empty" => forbidden,
                    "redundant" => duplicate,
                    _ => 0,
                };
                let mut transitions = vec![Transition::new("cons", [head, tail])];
                if self.family == "overlap" {
                    transitions.push(Transition::new("cons", [leaf, tail]));
                }
                let id = states.len();
                states.push(transitions);
                tail = id;
            }
            *filter = tail;
        }
        (Grammar::new(states).unwrap(), source, filters)
    }
}
fn bit_path(index: usize) -> Vec<usize> {
    let mut p = vec![1; index];
    p.push(0);
    p
}
fn at<'a>(mut term: &'a Term, path: &[usize]) -> Option<&'a Term> {
    for &i in path {
        let Term::App(_, args) = term else {
            return None;
        };
        term = args.get(i)?;
    }
    Some(term)
}
pub fn member(g: &Grammar, state: usize, term: &Term) -> bool {
    let Term::App(name, args) = term else {
        return false;
    };
    g.states()[state].iter().any(|tr| {
        tr.symbol == *name
            && tr.children.len() == args.len()
            && tr
                .children
                .iter()
                .zip(args)
                .all(|(&child, arg)| member(g, child, arg))
    })
}
pub fn accepted(g: &Grammar, request: &Request, term: &Term) -> bool {
    request.roots.iter().all(|&r| member(g, r, term))
        && request
            .equalities
            .iter()
            .all(|(a, b)| match (at(term, a), at(term, b)) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            })
}
// Dynamic enumeration memoizes each state and deduplicates membership values.
// It does not use the lazy solver's position graph or propagation.
fn enumerate(g: &Grammar, root: usize) -> Vec<Term> {
    fn state(g: &Grammar, id: usize, memo: &mut BTreeMap<usize, Rc<Vec<Term>>>) -> Rc<Vec<Term>> {
        if let Some(values) = memo.get(&id) {
            return values.clone();
        }
        let mut out = BTreeSet::new();
        for tr in &g.states()[id] {
            let mut tuples = vec![vec![]];
            for &child in &tr.children {
                let values = state(g, child, memo);
                tuples = tuples
                    .into_iter()
                    .flat_map(|args| {
                        let values = values.clone();
                        (0..values.len()).map(move |i| {
                            let mut next = args.clone();
                            next.push(values[i].clone());
                            next
                        })
                    })
                    .collect();
            }
            out.extend(tuples.into_iter().map(|args| t(&tr.symbol, args)));
        }
        let out = Rc::new(out.into_iter().collect::<Vec<_>>());
        memo.insert(id, out.clone());
        out
    }
    let mut memo = BTreeMap::new();
    let values = state(g, root, &mut memo);
    drop(memo);
    Rc::try_unwrap(values).unwrap()
}
fn estimate(g: &Grammar, root: usize, memo: &mut BTreeMap<usize, u128>) -> u128 {
    if let Some(&n) = memo.get(&root) {
        return n;
    }
    let mut n = 0u128;
    for tr in g.states()[root].iter().collect::<BTreeSet<_>>() {
        let mut product = 1u128;
        for &child in &tr.children {
            product = product.saturating_mul(estimate(g, child, memo));
        }
        n = n.saturating_add(product);
    }
    memo.insert(root, n);
    n
}
impl Prepared {
    pub fn new(mode: &str, schema: Schema) -> Self {
        assert!(MODES.contains(&mode));
        let (grammar, source, filters) = schema.program();
        let grammar = if mode == "reduced" {
            grammar.reduce_membership()
        } else {
            grammar
        };
        Self {
            grammar: if mode == "lowered" {
                None
            } else {
                Some(grammar)
            },
            schema,
            mode: mode.into(),
            source,
            filters,
            cache: RefCell::new(BTreeMap::new()),
        }
    }
    pub fn request(&self, index: usize) -> Request {
        let parity = index % 2;
        let roots = if matches!(self.schema.family, "all" | "equal") {
            vec![self.source]
        } else {
            vec![self.source, self.filters[parity]]
        };
        let mut equalities = vec![];
        if self.schema.family == "equal" && self.schema.width > 0 {
            let anchor = if parity == 0 {
                0
            } else {
                self.schema.width - 1
            };
            for i in 0..self.schema.width {
                if i != anchor {
                    equalities.push((bit_path(anchor), bit_path(i)));
                }
            }
        } else if self.schema.width > 0 {
            let i = if parity == 0 {
                0
            } else {
                self.schema.width - 1
            };
            equalities.push((bit_path(i), bit_path(i)));
        }
        Request { roots, equalities }
    }
    pub fn start(&self, request: Request) -> Running<'_> {
        match self.mode.as_str() {
            "lazy" | "reduced" => {
                Running::Lazy(Search::new(self.grammar.as_ref().unwrap(), request).unwrap())
            }
            "enumerate" | "enum-reuse" => {
                let mut estimates = BTreeMap::new();
                let root = *request
                    .roots
                    .iter()
                    .min_by_key(|&&r| estimate(self.grammar.as_ref().unwrap(), r, &mut estimates))
                    .unwrap();
                let values = if self.mode == "enum-reuse" {
                    self.cache
                        .borrow_mut()
                        .entry(root)
                        .or_insert_with(|| Rc::new(enumerate(self.grammar.as_ref().unwrap(), root)))
                        .clone()
                } else {
                    Rc::new(enumerate(self.grammar.as_ref().unwrap(), root))
                };
                Running::Enumerate {
                    grammar: self.grammar.as_ref().unwrap(),
                    request,
                    values,
                    cursor: 0,
                }
            }
            "lowered" => {
                // Hand-derived control accepts exactly this prepared schema's queries.
                let parity = if request == self.request(0) {
                    0
                } else {
                    assert_eq!(request, self.request(1));
                    1
                };
                let n = self.schema.width;
                let groups = (0..n)
                    .map(|i| if self.schema.family == "equal" { 0 } else { i })
                    .collect::<Vec<_>>();
                let count = if n == 0 {
                    0
                } else if self.schema.family == "equal" {
                    1
                } else {
                    n
                };
                let choices = (0..count)
                    .map(|i| match self.schema.family {
                        "empty" => vec![],
                        "redundant" => vec![parity != 0],
                        "selective" if i + 1 < n => vec![parity != 0],
                        _ => vec![false, true],
                    })
                    .collect::<Vec<_>>();
                let done = choices.iter().any(Vec::is_empty);
                Running::Lowered {
                    width: n,
                    groups,
                    choices,
                    cursor: vec![0; count],
                    done,
                }
            }
            _ => unreachable!(),
        }
    }
}
impl Running<'_> {
    pub fn tick(&mut self) -> Event {
        match self {
            Self::Lazy(search) => {
                let mut b = search.advance(1).unwrap();
                if let Some(a) = b.answers.pop() {
                    Event::Answer(a)
                } else if b.exhausted {
                    Event::Done
                } else {
                    Event::Progress
                }
            }
            Self::Enumerate {
                grammar,
                request,
                values,
                cursor,
            } => {
                if *cursor == values.len() {
                    return Event::Done;
                }
                let term = &values[*cursor];
                *cursor += 1;
                if accepted(grammar, request, term) {
                    Event::Answer(Answer {
                        term: term.clone(),
                        multiplicity: grammar.multiplicity(request.roots[0], term).unwrap(),
                    })
                } else {
                    Event::Progress
                }
            }
            Self::Lowered {
                width,
                groups,
                choices,
                cursor,
                done,
            } => {
                if *done {
                    return Event::Done;
                }
                let term = (0..*width).rev().fold(atom("nil"), |tail, i| {
                    t(
                        "cons",
                        [
                            atom(if choices[groups[i]][cursor[groups[i]]] {
                                "b"
                            } else {
                                "a"
                            }),
                            tail,
                        ],
                    )
                });
                let mut advanced = false;
                for i in (0..cursor.len()).rev() {
                    cursor[i] += 1;
                    if cursor[i] < choices[i].len() {
                        advanced = true;
                        break;
                    }
                    cursor[i] = 0;
                }
                *done = !advanced;
                Event::Answer(Answer {
                    term,
                    multiplicity: 1,
                })
            }
        }
    }
}
