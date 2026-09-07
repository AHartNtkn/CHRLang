use super::*;
type Pred = (String, usize);
fn pred(c: &Constraint) -> Pred {
    (c.name.clone(), c.args.len())
}
fn goals(g: &Goal, out: &mut Vec<Pred>) {
    match g {
        Goal::Constraint(c) => out.push(pred(c)),
        Goal::And(gs) => {
            for g in gs {
                goals(g, out)
            }
        }
        Goal::Or(a, b) => {
            goals(a, out);
            goals(b, out)
        }
        _ => {}
    }
}
fn vars(t: &Term, out: &mut BTreeSet<Var>, stats: &mut Stats) {
    stats.certificate_terms += 1;
    match t {
        Term::Var(v) => {
            out.insert(*v);
        }
        Term::App(_, args) => {
            for a in args {
                vars(a, out, stats)
            }
        }
    }
}
fn root(parents: &mut [usize], i: usize) -> usize {
    if parents[i] != i {
        parents[i] = root(parents, parents[i]);
    }
    parents[i]
}
fn unite(parents: &mut [usize], a: usize, b: usize, stats: &mut Stats) {
    stats.certificate_edges += 1;
    let a = root(parents, a);
    let b = root(parents, b);
    if a != b {
        parents[a.max(b)] = a.min(b);
    }
}
pub(super) fn regions(
    rules: Vec<Rule>,
    query: Query,
    stats: &mut Stats,
) -> Vec<(Vec<Rule>, Query)> {
    let mut all = BTreeSet::new();
    let mut links = vec![];
    for r in &rules {
        let mut ps = r
            .kept
            .iter()
            .chain(&r.removed)
            .map(pred)
            .collect::<Vec<_>>();
        goals(&r.body, &mut ps);
        all.extend(ps.iter().cloned());
        links.push(ps);
    }
    all.extend(query.constraints.iter().map(pred));
    stats.certificate_predicates = all.len();
    let ids = all
        .into_iter()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<BTreeMap<_, _>>();
    let mut parents = (0..ids.len()).collect::<Vec<_>>();
    for link in links {
        for p in link.iter().skip(1) {
            unite(&mut parents, ids[&link[0]], ids[p], stats);
        }
    }
    let mut owners = BTreeMap::new();
    for c in &query.constraints {
        let id = ids[&pred(c)];
        let mut seen = BTreeSet::new();
        for t in &c.args {
            vars(t, &mut seen, stats);
        }
        for v in seen {
            if let Some(other) = owners.insert(v, id) {
                unite(&mut parents, id, other, stats);
            }
        }
    }
    let mut regions: BTreeMap<usize, (Vec<Rule>, Query)> = BTreeMap::new();
    let empty = || {
        (
            vec![],
            Query {
                constraints: vec![],
                outputs: vec![],
            },
        )
    };
    for c in query.constraints {
        let id = root(&mut parents, ids[&pred(&c)]);
        regions
            .entry(id)
            .or_insert_with(empty)
            .1
            .constraints
            .push(c);
    }
    for (name, v) in query.outputs {
        let id = owners
            .get(&v)
            .map(|i| root(&mut parents, *i))
            .unwrap_or(ids.len());
        regions
            .entry(id)
            .or_insert_with(empty)
            .1
            .outputs
            .push((name, v));
    }
    for r in rules {
        let head = r.kept.first().or_else(|| r.removed.first()).unwrap();
        let id = root(&mut parents, ids[&pred(head)]);
        if let Some(region) = regions.get_mut(&id) {
            region.0.push(r);
        }
    }
    if regions.is_empty() {
        vec![empty()]
    } else {
        regions.into_values().collect()
    }
}
