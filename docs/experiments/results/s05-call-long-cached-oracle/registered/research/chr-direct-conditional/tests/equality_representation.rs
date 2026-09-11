use chr_relational::contextual::Store;

#[test]
fn equal_ground_values_need_not_share_a_canonical_root() {
    let mut s = Store::default();
    let x = s.unknown();
    let alias_atom = s.constructor("a", &[]);
    let item_atom = s.constructor("a", &[]);
    s.equate(x, alias_atom);
    assert!(s.step());
    let values = s.export(&[x, item_atom]).unwrap();
    assert_eq!(values[0], values[1]);
    assert_ne!(s.root(x), s.root(item_atom));
    s.equate(x, item_atom);
    assert!(s.step());
    assert_eq!(s.root(x), s.root(item_atom));
    // A structural no-op can still merge canonical identities.
}

#[test]
fn repeated_alias_equation_has_no_canonical_change() {
    let mut s = Store::default();
    let x = s.unknown();
    let y = s.unknown();
    s.equate(x, y);
    assert!(s.step());
    let root = s.root(x);
    assert_eq!(root, s.root(y));
    s.equate(x, y);
    assert!(s.step());
    assert_eq!(root, s.root(x));
    assert_eq!(root, s.root(y));
    assert_eq!(s.pending(), 0);
}
