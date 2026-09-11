use chr_structural::{
    Datum,
    regular::{Automaton, Stats, Transition},
};
#[test]
fn cyclic_grammars_denote_finite_trees_and_intersection_is_exact() {
    let n = Automaton::new(
        vec![vec![
            Transition::new("k", []),
            Transition::new("s", []),
            Transition::new("a", [0, 0]),
        ]],
        0,
    )
    .unwrap();
    let k = Automaton::new(
        vec![vec![Transition::new("k", []), Transition::new("a", [0, 0])]],
        0,
    )
    .unwrap();
    let s = Automaton::new(
        vec![vec![Transition::new("s", []), Transition::new("a", [0, 0])]],
        0,
    )
    .unwrap();
    let mut stats = Stats::default();
    let nk = n.intersect(&k, &mut stats);
    assert_eq!(nk.enumerate(9, &mut stats), k.enumerate(9, &mut stats));
    assert_eq!(n.enumerate(9, &mut stats).len(), 550);
    assert_eq!(k.enumerate(9, &mut stats).len(), 23);
    let empty = k.intersect(&s, &mut stats);
    assert!(empty.witness(&mut stats).is_none());
    assert!(empty.enumerate(9, &mut stats).is_empty());
    assert!(n.witness(&mut stats).is_some());
    assert_eq!(n.contains(&Datum::var(0), &mut stats), None);
    assert_eq!(
        n.contains(&Datum::app("unknown", []), &mut stats),
        Some(false)
    );
}

#[test]
fn generated_small_automata_intersect_pointwise_on_independent_test_trees() {
    use chr_syntax::{Term, atom, t};
    let mut levels = vec![vec![], vec![atom("z")]];
    for n in 2..=6 {
        let mut level = levels[n - 1]
            .iter()
            .map(|x| t("s", [x.clone()]))
            .collect::<Vec<_>>();
        for left in 1..n - 1 {
            for a in &levels[left] {
                for b in &levels[n - 1 - left] {
                    level.push(t("a", [a.clone(), b.clone()]));
                }
            }
        }
        levels.push(level);
    }
    let trees = levels.into_iter().flatten().collect::<Vec<Term>>();
    let build = |mask: u16| {
        Automaton::new(
            (0..2)
                .map(|state| {
                    let candidates = [
                        Transition::new("z", []),
                        Transition::new("s", [0]),
                        Transition::new("s", [1]),
                        Transition::new("a", [0, 1]),
                    ];
                    candidates
                        .into_iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << (4 * state + i)) != 0)
                        .map(|(_, t)| t)
                        .collect()
                })
                .collect(),
            0,
        )
        .unwrap()
    };
    for i in 0..256 {
        let a = build(i);
        let b = build((i * 73 + 19) % 256);
        let mut stats = Stats::default();
        let intersection = a.intersect(&b, &mut stats);
        for tree in &trees {
            assert_eq!(
                intersection.contains_term(tree, &mut stats),
                Some(
                    a.contains_term(tree, &mut stats).unwrap()
                        && b.contains_term(tree, &mut stats).unwrap()
                )
            );
        }
        if let Some(witness) = intersection.witness(&mut stats) {
            assert_eq!(a.contains(&witness, &mut stats), Some(true));
            assert_eq!(b.contains(&witness, &mut stats), Some(true));
        }
    }
}
