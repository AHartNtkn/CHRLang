use chr_direct_choice::{Context, Graph, Resources, View};
use chr_syntax::{atom, t, v};

#[test]
fn context_algebra_matches_boolean_truth_tables() {
    let mut graph = Graph::default();
    let labels: Vec<_> = (0..4).map(|_| graph.birth(Context::all())).collect();
    let contexts: Vec<_> = (0..81)
        .map(|mut code| {
            let mut context = Context::all();
            let mut specification = [None; 4];
            for (index, label) in labels.iter().enumerate() {
                let digit = code % 3;
                code /= 3;
                if digit != 0 {
                    specification[index] = Some(digit == 2);
                    context = context.select(*label, digit == 2).unwrap();
                }
            }
            (context, specification)
        })
        .collect();
    for (a, sa) in &contexts {
        for (b, sb) in &contexts {
            let intersection = a.intersection(b);
            let difference = a.subtract(b);
            for bits in 0..16 {
                let values: Vec<_> = (0..4).map(|i| bits & (1 << i) != 0).collect();
                let holds = |spec: &[Option<bool>; 4]| {
                    spec.iter()
                        .zip(&values)
                        .all(|(x, y)| x.is_none_or(|v| v == *y))
                };
                assert_eq!(a.contains_assignment(&values), holds(sa));
                assert_eq!(
                    intersection
                        .as_ref()
                        .is_some_and(|c| c.contains_assignment(&values)),
                    holds(sa) && holds(sb)
                );
                assert_eq!(
                    difference
                        .iter()
                        .filter(|c| c.contains_assignment(&values))
                        .count(),
                    usize::from(holds(sa) && !holds(sb))
                );
            }
        }
    }
}

#[test]
fn opaque_access_preserves_correlated_and_independent_choices() {
    let mut g = Graph::default();
    let first = g.birth(Context::all());
    let second = g.birth(Context::all());
    let a = g.app("a", vec![]);
    let b = g.app("b", vec![]);
    let x = g.choice(first, a, b);
    let y = g.choice(second, a, b);
    let h = g.unknown(7);
    let pair = g.app("pair", vec![x, x, y, h, h]);
    assert_eq!(
        g.expose(pair, &Context::all()),
        vec![(
            Context::all(),
            View::Constructor("pair".into(), vec![x, x, y, h, h])
        )]
    );
    let mut actual: Vec<_> = g
        .observe(&[pair], &Context::all())
        .into_iter()
        .map(|(_, v)| v)
        .collect();
    actual.sort();
    let mut expected = vec![];
    for x in [atom("a"), atom("b")] {
        for y in [atom("a"), atom("b")] {
            expected.push(vec![t("pair", vec![x.clone(), x.clone(), y, v(7), v(7)])]);
        }
    }
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn dynamic_births_preserve_raw_multiplicity_without_inactive_assignments() {
    let mut g = Graph::default();
    let outer = g.birth(Context::all());
    let right = Context::all().select(outer, true).unwrap();
    let inner = g.birth(right);
    let a = g.app("a", vec![]);
    let b = g.app("b", vec![]);
    let c = g.app("c", vec![]);
    let nested = g.choice(inner, b, c);
    let root = g.choice(outer, a, nested);
    let actual: Vec<_> = g
        .observe(&[root], &Context::all())
        .into_iter()
        .map(|(_, v)| v)
        .collect();
    assert_eq!(
        actual,
        vec![vec![atom("a")], vec![atom("b")], vec![atom("c")]]
    );
    assert_eq!(g.histories(&Context::all()).len(), 3);
    // Choice is a source effect even when no output refers to its result.
    assert_eq!(g.observe(&[], &Context::all()).len(), 3);
    let duplicate = g.birth(Context::all());
    let same = g.choice(duplicate, a, a);
    assert_eq!(g.observe(&[same], &Context::all()).len(), 6);
    // A request carrying an inactive descendant cannot describe a valid history.
    let impossible = Context::all()
        .select(outer, false)
        .unwrap()
        .select(inner, true)
        .unwrap();
    assert!(g.histories(&impossible).is_empty());
}

#[test]
fn conflicting_consumers_and_late_contexts_match_scalar_resources() {
    let mut g = Graph::default();
    let labels: Vec<_> = (0..3).map(|_| g.birth(Context::all())).collect();
    let all = Context::all();
    let left = all.select(labels[0], false).unwrap();
    let second = all.select(labels[1], true).unwrap();
    let last = all.select(labels[2], false).unwrap();
    let mut resources = Resources::default();
    let ids: Vec<_> = (0..3).map(|_| resources.post(all.clone())).collect();
    let mut scalar = [[true; 3]; 8];
    // Includes overlap, disjoint late arrivals, failed atomic joins and duplicate heads.
    let script = [
        (vec![0], &left, 0),
        (vec![0, 1], &second, 1),
        (vec![0], &all, 2),
        (vec![1, 2], &last, 3),
        (vec![2, 2], &all, 2),
        (vec![1, 2], &all, 2),
    ];
    for (indices, context, selector) in script {
        let request: Vec<_> = indices.iter().map(|i| ids[*i]).collect();
        let granted = resources.consume(&request, context);
        for (bits, live) in scalar.iter_mut().enumerate() {
            let values: Vec<_> = (0..3).map(|i| bits & (1 << i) != 0).collect();
            let applies = match selector {
                0 => !values[0],
                1 => values[1],
                2 => true,
                3 => !values[2],
                _ => unreachable!(),
            };
            let unique = indices
                .iter()
                .enumerate()
                .all(|(i, x)| !indices[..i].contains(x));
            let succeeds = applies && unique && indices.iter().all(|i| live[*i]);
            assert_eq!(
                granted
                    .iter()
                    .filter(|c| c.contains_assignment(&values))
                    .count(),
                usize::from(succeeds)
            );
            if succeeds {
                for i in &indices {
                    live[*i] = false;
                }
            }
            for (i, id) in ids.iter().enumerate() {
                assert_eq!(
                    resources
                        .available(*id)
                        .iter()
                        .filter(|c| c.contains_assignment(&values))
                        .count(),
                    usize::from(live[i])
                );
            }
        }
    }
    // A newly posted, equal-valued source occurrence has independent identity.
    let fresh = resources.post(all.clone());
    assert_ne!(fresh, ids[0]);
    assert_eq!(resources.consume(&[fresh], &all), vec![all]);
}

#[test]
fn constructor_demand_consumes_only_matching_contexts_then_accepts_late_births() {
    let mut g = Graph::default();
    let outer = g.birth(Context::all());
    let a = g.app("a", vec![]);
    let b = g.app("b", vec![]);
    let value = g.choice(outer, a, b);
    let mut resources = Resources::default();
    let token = resources.post(Context::all());
    let task = resources.post(Context::all());
    // An a-headed consumer demands only the selected value, then takes two heads.
    let mut first = Vec::new();
    for (context, view) in g.expose(value, &Context::all()) {
        if matches!(view, View::Constructor(ref name, _) if name == "a") {
            first.extend(resources.consume(&[token, task], &context));
        }
    }
    assert_eq!(first.len(), 1);
    assert_eq!(
        g.observe(&[value, value], &first[0])[0].1,
        vec![atom("a"), atom("a")]
    );
    // A late competing consumer gets b only, even when it arrives with no choices demanded.
    let remainder = resources.consume(&[token, task], &Context::all());
    assert_eq!(remainder.len(), 1);
    assert_eq!(
        g.observe(&[value, value], &remainder[0])[0].1,
        vec![atom("b"), atom("b")]
    );
    assert!(resources.consume(&[token], &Context::all()).is_empty());
    // The successful a application now introduces its own local choice and occurrence.
    let late = g.birth(first[0].clone());
    let output = g.choice(late, a, b);
    let fresh = resources.post(first[0].clone());
    let regions = resources.consume(&[fresh], &Context::all());
    assert_eq!(regions, first);
    let answers: Vec<_> = g
        .observe(&[value, output], &regions[0])
        .into_iter()
        .map(|(_, terms)| terms)
        .collect();
    assert_eq!(
        answers,
        vec![vec![atom("a"), atom("a")], vec![atom("a"), atom("b")]]
    );
    // The already consumed parent cannot be consumed again by either new descendant.
    for context in g.histories(&regions[0]) {
        assert!(resources.consume(&[task], &context).is_empty());
    }
    assert_eq!(g.histories(&Context::all()).len(), 3);
}

#[test]
#[should_panic(expected = "birth activation omits a causal ancestor")]
fn conditional_birth_rejects_missing_causal_ancestor() {
    let mut g = Graph::default();
    let outer = g.birth(Context::all());
    let inner = g.birth(Context::all().select(outer, true).unwrap());
    g.birth(Context::all().select(inner, false).unwrap());
}
