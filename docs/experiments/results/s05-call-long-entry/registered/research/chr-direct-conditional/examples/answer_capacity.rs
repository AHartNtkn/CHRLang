#[allow(dead_code)]
#[path = "../tests/composition_support/mod.rs"]
mod engines;
#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
mod scalar;
#[path = "support/broad_mixed_source.rs"]
mod source;
use chr_syntax::{Answer, Constraint, Term};
#[derive(Default, Debug)]
struct Bytes {
    owned: usize,
    spare_fields: usize,
    spare_other: usize,
}
fn vector<T>(v: &Vec<T>, fields: bool, b: &mut Bytes) {
    b.owned += v.capacity() * std::mem::size_of::<T>();
    let extra = (v.capacity() - v.len()) * std::mem::size_of::<T>();
    if fields {
        b.spare_fields += extra
    } else {
        b.spare_other += extra
    }
}
fn term(t: &Term, b: &mut Bytes) {
    if let Term::App(n, xs) = t {
        b.owned += n.capacity();
        vector(xs, true, b);
        for x in xs {
            term(x, b)
        }
    }
}
fn constraint(c: &Constraint, b: &mut Bytes) {
    b.owned += c.name.capacity();
    vector(&c.args, false, b);
    for x in &c.args {
        term(x, b)
    }
}
fn measure(xs: &Vec<Answer>) -> Bytes {
    let mut b = Bytes::default();
    vector(xs, false, &mut b);
    for a in xs {
        vector(&a.outputs, false, &mut b);
        for (n, t) in &a.outputs {
            b.owned += n.capacity();
            term(t, &mut b)
        }
        vector(&a.residual, false, &mut b);
        for c in &a.residual {
            constraint(c, &mut b)
        }
    }
    b
}
fn main() {
    let rules = source::rules("deep");
    let q = source::query("deep", 3, 0);
    let expected = source::expected("deep", 3, 0);
    for (mode, name) in [(0, "scan"), (1, "contextual"), (5, "conditional")] {
        let answers = engines::Engine::new(mode, &rules, &q).collect();
        let b = measure(&answers);
        scalar::same_raw(answers, expected.clone());
        println!("{name}: {b:?}; four_query_owned={}", 4 * b.owned);
    }
}
