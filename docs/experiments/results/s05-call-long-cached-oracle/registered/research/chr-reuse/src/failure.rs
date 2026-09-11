//! Sufficient structural contradiction certificates, checked on current operands.
//! A CHR driver may use these only for an equation actually being executed.
use chr_syntax::{Term, Var};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Certificate {
    Clash {
        pair: Vec<usize>,
    },
    Occurs {
        pair: Vec<usize>,
        variable_on_left: bool,
        occurrence: Vec<usize>,
    },
}
#[derive(Default, Debug)]
pub struct Stats {
    pub discovery_nodes: u64,
    pub check_nodes: u64,
    pub checks: u64,
    pub learned: u64,
    pub hits: u64,
}
/// The checker follows only the claimed paths; it does not invoke discovery or unification.
pub fn verify(proof: &Certificate, left: &Term, right: &Term, stats: &mut Stats) -> bool {
    stats.checks += 1;
    let path = match proof {
        Certificate::Clash { pair } | Certificate::Occurs { pair, .. } => pair,
    };
    let (mut a, mut b) = (left, right);
    for &index in path {
        stats.check_nodes += 1;
        let (Term::App(n, x), Term::App(m, y)) = (a, b) else {
            return false;
        };
        if n != m || x.len() != y.len() || index >= x.len() {
            return false;
        }
        a = &x[index];
        b = &y[index];
    }
    stats.check_nodes += 1;
    match proof {
        Certificate::Clash { .. } => {
            matches!((a,b),(Term::App(n,x),Term::App(m,y)) if n!=m || x.len()!=y.len())
        }
        Certificate::Occurs {
            variable_on_left,
            occurrence,
            ..
        } => {
            let (variable, mut term) = if *variable_on_left { (a, b) } else { (b, a) };
            let Term::Var(v) = variable else {
                return false;
            };
            if occurrence.is_empty() {
                return false;
            }
            for &index in occurrence {
                stats.check_nodes += 1;
                let Term::App(_, args) = term else {
                    return false;
                };
                let Some(child) = args.get(index) else {
                    return false;
                };
                term = child;
            }
            matches!(term,Term::Var(w) if v==w)
        }
    }
}
fn occurrence(v: Var, t: &Term, stats: &mut Stats) -> Option<Vec<usize>> {
    let mut stack = vec![(t, vec![])];
    while let Some((term, path)) = stack.pop() {
        stats.discovery_nodes += 1;
        match term {
            Term::Var(w) if v == *w => return Some(path),
            Term::App(_, args) => {
                for (i, child) in args.iter().enumerate().rev() {
                    let mut next = path.clone();
                    next.push(i);
                    stack.push((child, next));
                }
            }
            _ => {}
        }
    }
    None
}
pub fn discover(left: &Term, right: &Term, stats: &mut Stats) -> Option<Certificate> {
    let mut stack = vec![(left, right, vec![])];
    while let Some((a, b, path)) = stack.pop() {
        stats.discovery_nodes += 1;
        match (a, b) {
            (Term::App(n, x), Term::App(m, y)) => {
                if n != m || x.len() != y.len() {
                    return Some(Certificate::Clash { pair: path });
                }
                for (i, (a, b)) in x.iter().zip(y).enumerate().rev() {
                    let mut next = path.clone();
                    next.push(i);
                    stack.push((a, b, next));
                }
            }
            (Term::Var(v), t @ Term::App(..)) | (t @ Term::App(..), Term::Var(v)) => {
                if let Some(found) = occurrence(*v, t, stats) {
                    return Some(Certificate::Occurs {
                        pair: path,
                        variable_on_left: matches!(a, Term::Var(_)),
                        occurrence: found,
                    });
                }
            }
            _ => {}
        }
    }
    None
}
#[derive(Default)]
pub struct Learner {
    certificates: Vec<Certificate>,
    stats: Stats,
}
impl Learner {
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn proves_failure(&mut self, left: &Term, right: &Term) -> bool {
        for proof in &self.certificates {
            if verify(proof, left, right, &mut self.stats) {
                self.stats.hits += 1;
                return true;
            }
        }
        false
    }
    /// Called after the ordinary equation operation fails. Discovery alone does
    /// not establish validity: a separately implemented checker certifies it.
    pub fn learn(&mut self, left: &Term, right: &Term) {
        if let Some(proof) = discover(left, right, &mut self.stats) {
            assert!(
                verify(&proof, left, right, &mut self.stats),
                "invalid discovered proof"
            );
            if !self.certificates.contains(&proof) {
                self.certificates.push(proof);
                self.stats.learned += 1;
            }
        }
    }
}

/// A second checker over observations from the current source equation. The
/// reader must reject a path crossing a variable or an out-of-range child.
pub fn verify_paths(
    proof: &Certificate,
    mut read: impl FnMut(bool, &[usize]) -> Option<chr_persistent::continuations::TermHead>,
    stats: &mut Stats,
) -> bool {
    use chr_persistent::continuations::TermHead::{Constructor, Variable};
    stats.checks += 1;
    let pair = match proof {
        Certificate::Clash { pair } | Certificate::Occurs { pair, .. } => pair,
    };
    for (depth, &index) in pair.iter().enumerate() {
        stats.check_nodes += 1;
        let (Some(Constructor(a, n)), Some(Constructor(b, m))) =
            (read(true, &pair[..depth]), read(false, &pair[..depth]))
        else {
            return false;
        };
        if a != b || n != m || index >= n {
            return false;
        }
    }
    stats.check_nodes += 1;
    let (a, b) = (read(true, pair), read(false, pair));
    match proof {
        Certificate::Clash { .. } => {
            matches!((a,b),(Some(Constructor(a,n)),Some(Constructor(b,m))) if a!=b || n!=m)
        }
        Certificate::Occurs {
            variable_on_left,
            occurrence,
            ..
        } => {
            if occurrence.is_empty() {
                return false;
            }
            let Some(Variable(v)) = (if *variable_on_left { a } else { b }) else {
                return false;
            };
            let mut full = pair.clone();
            full.extend(occurrence);
            stats.check_nodes += occurrence.len() as u64;
            matches!(read(!*variable_on_left,&full),Some(Variable(w)) if v==w)
        }
    }
}
impl Learner {
    pub fn proves_failure_paths(
        &mut self,
        mut read: impl FnMut(bool, &[usize]) -> Option<chr_persistent::continuations::TermHead>,
    ) -> bool {
        for proof in &self.certificates {
            if verify_paths(proof, &mut read, &mut self.stats) {
                self.stats.hits += 1;
                return true;
            }
        }
        false
    }
}
