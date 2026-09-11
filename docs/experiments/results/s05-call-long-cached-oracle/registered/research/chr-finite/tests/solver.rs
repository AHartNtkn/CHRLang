use chr_finite::{
    cnf::{Cnf, Encoding},
    model::{Endpoint, Forbidden, Problem, QueryDomains},
    native::Prepared as NativePrepared,
    z3,
};
fn assignments(p: &Problem, givens: &[(Endpoint, u8)]) -> Vec<Vec<u8>> {
    let mut result = vec![];
    for encoded in 0..3usize.pow(p.variables() as u32) {
        let mut code = encoded;
        let row: Vec<_> = (0..p.variables())
            .map(|_| {
                let a = (code % 3) as u8;
                code /= 3;
                a
            })
            .collect();
        let value = |x: Endpoint| match x {
            Endpoint::Var(v) => row[v],
            Endpoint::Value(a) => a,
        };
        if givens.iter().any(|(v, a)| value(*v) != *a)
            || p.forbidden()
                .iter()
                .any(|f| value(f.left) == f.a && value(f.right) == f.b)
        {
            continue;
        }
        result.push(row)
    }
    result.sort();
    result
}
#[test]
fn boolean_and_native_models_preserve_every_binary_table() {
    for bits in 0..512 {
        let forbidden = (0..9)
            .filter(|i| bits & (1 << i) != 0)
            .map(|i| Forbidden {
                left: Endpoint::Var(0),
                right: Endpoint::Var(1),
                a: i / 3,
                b: i % 3,
            })
            .collect();
        let p = Problem::new(2, forbidden).unwrap();
        let native_prepared = NativePrepared::new(&p);
        for encoding in [Encoding::Support, Encoding::Conflict] {
            let mut prepared = z3::Prepared::new(&Cnf::encode(&p, encoding)).unwrap();
            for givens in [
                vec![],
                vec![(Endpoint::Var(0), 0)],
                vec![(Endpoint::Var(0), 1)],
                vec![(Endpoint::Var(0), 2)],
                vec![(Endpoint::Var(1), 0)],
                vec![(Endpoint::Var(1), 1)],
                vec![(Endpoint::Var(1), 2)],
            ] {
                let expected = assignments(&p, &givens);
                let mut native: Vec<_> = native_prepared.start(&givens).unwrap().collect();
                native.sort();
                assert_eq!(native, expected);
                let mut query = prepared.start(&p.domains(&givens).unwrap()).unwrap();
                let mut actual = vec![];
                while let Some(row) = query.next_solution().unwrap() {
                    actual.push(row)
                }
                actual.sort();
                assert_eq!(actual, expected, "table {bits} {encoding:?}");
            }
        }
    }
}
#[test]
fn scoped_enumeration_survives_early_drop_and_ground_contradictions() {
    for n in [0, 2] {
        let p = Problem::new(n, vec![]).unwrap();
        let mut prepared = z3::Prepared::new(&Cnf::encode(&p, Encoding::Support)).unwrap();
        for _ in 0..3 {
            let mut q = prepared.start(&p.domains(&[]).unwrap()).unwrap();
            assert!(q.next_solution().unwrap().is_some());
        }
        let false_query = p.domains(&[(Endpoint::Value(0), 1)]).unwrap();
        assert!(false_query.inconsistent);
        assert!(
            prepared
                .start(&false_query)
                .unwrap()
                .next_solution()
                .unwrap()
                .is_none()
        );
        let mut q = prepared.start(&p.domains(&[]).unwrap()).unwrap();
        let mut raw = 0;
        while q.next_solution().unwrap().is_some() {
            raw += 1
        }
        assert_eq!(raw, 3usize.pow(n as u32));
    }
    assert!(
        z3::Prepared::new(&Cnf {
            variables: 3,
            clauses: vec![vec![4]]
        })
        .is_err()
    );
    let p = Problem::new(1, vec![]).unwrap();
    let mut z = z3::Prepared::new(&Cnf::encode(&p, Encoding::Support)).unwrap();
    assert!(
        z.start(&QueryDomains {
            masks: vec![8],
            inconsistent: false
        })
        .is_err()
    );
}
#[test]
fn support_unit_propagation_matches_arc_consistency_for_every_table_and_domain_pair() {
    for bits in 0..512 {
        let forbidden = (0..9)
            .filter(|i| bits & (1 << i) != 0)
            .map(|i| Forbidden {
                left: Endpoint::Var(0),
                right: Endpoint::Var(1),
                a: i / 3,
                b: i % 3,
            })
            .collect();
        let p = Problem::new(2, forbidden).unwrap();
        let cnf = Cnf::encode(&p, Encoding::Support);
        for x in 1..8 {
            for y in 1..8 {
                let mut supported = vec![0, 0];
                for a in 0..3 {
                    for b in 0..3 {
                        if x & (1 << a) != 0 && y & (1 << b) != 0 && bits & (1 << (3 * a + b)) == 0
                        {
                            supported[0] |= 1 << a;
                            supported[1] |= 1 << b;
                        }
                    }
                }
                let expected = (!supported.contains(&0)).then_some(supported);
                let mut domains = vec![x, y];
                let consistent = chr_finite::native::arc_consistency(&p, &mut domains);
                let unit = chr_finite::cnf::unit_propagate(&cnf, &[x, y]);
                assert_eq!(unit, expected, "independent support projection");
                assert_eq!(
                    unit,
                    consistent.then_some(domains),
                    "table={bits} domains={x},{y}"
                );
            }
        }
    }
}
