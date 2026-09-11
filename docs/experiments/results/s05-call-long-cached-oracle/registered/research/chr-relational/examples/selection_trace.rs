//! Independent source-rule attribution after the ownership comparison.
#[path = "../tests/support/chr_forest.rs"]
mod forest;
#[path = "../tests/support/chr_constructors.rs"]
mod kernel;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../tests/support/chr_selection.rs"]
mod selection;
#[path = "../tests/support/selection_source.rs"]
mod source;
fn main() {
    for reserved in [false, true] {
        let rules = if reserved {
            selection::reserved_rules()
        } else {
            selection::rules(true)
        };
        for family in ["flat", "sparse", "chain", "competition"] {
            for n in [2, 6] {
                for value in ["a", "b"] {
                    let query = source::query(family, n, value);
                    let encoded = source::encode(&query);
                    let expected = oracle::run(&[source::rule()], &query, 2_000_000);
                    let rows = oracle::run_traced(&rules, &encoded.query, 2_000_000);
                    assert_eq!(rows.len(), 1);
                    oracle::same_raw(vec![source::decode(rows[0].0.clone(), &encoded)], expected);
                    let mut counts = std::collections::BTreeMap::new();
                    for &i in &rows[0].1 {
                        *counts.entry(&rules[i].name).or_insert(0) += 1;
                    }
                    let counts = counts
                        .iter()
                        .map(|(name, n)| format!("\"{name}\":{n}"))
                        .collect::<Vec<_>>()
                        .join(",");
                    println!(
                        "{{\"reserved\":{reserved},\"family\":\"{family}\",\"size\":{n},\"value\":\"{value}\",\"applications\":{},\"rules\":{{{counts}}}}}",
                        rows[0].1.len()
                    );
                }
            }
        }
    }
}
