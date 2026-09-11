#[allow(dead_code)]
#[path = "../tests/composition_support/mod.rs"]
mod engines;
#[allow(dead_code)]
#[path = "support/effect_source.rs"]
mod source;
fn main() {
    for family in ["blocked", "rewrite", "writer"] {
        let s = source::Schema {
            family,
            work: 4,
            payload: 8,
            resource: true,
            fail_tail: false,
        };
        for mode in [0, 5] {
            for q in 0..4 {
                let a =
                    engines::Engine::new(mode, &s.rules(), &s.query(64 + q, q % 2 == 1)).collect();
                assert_eq!(a.len(), 1);
                println!(
                    "{family} mode={mode} query={q} residual_len={} residual_capacity={} slot_bytes={}",
                    a[0].residual.len(),
                    a[0].residual.capacity(),
                    std::mem::size_of::<chr_syntax::Constraint>()
                );
            }
        }
    }
}
