#[allow(dead_code)]
#[path = "../tests/composition_support/mod.rs"]
mod engines;
#[path = "support/order_source.rs"]
#[allow(dead_code)]
mod source;
fn main() {
    let schema = source::Schema {
        family: "oldest-first",
        resource: true,
        fail_tail: false,
        work: 4,
        payload: 8,
    };
    for mode in [0, 5, 7] {
        for q in 0..4 {
            let answers =
                engines::Engine::new(mode, &schema.rules(), &schema.query(8 + q, q % 2 == 1))
                    .collect();
            assert_eq!(answers.len(), 1);
            let a = &answers[0];
            println!(
                "mode={mode} query={q} outputs_len={} outputs_capacity={} residual_len={} residual_capacity={} constraint_bytes={}",
                a.outputs.len(),
                a.outputs.capacity(),
                a.residual.len(),
                a.residual.capacity(),
                std::mem::size_of::<chr_syntax::Constraint>()
            );
        }
    }
}
