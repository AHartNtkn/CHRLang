//! Independent denotation of the registered binary-list workload families.
use super::runtime::Schema;
use chr_syntax::{Term, atom, t};
pub fn expected(schema: Schema, query: usize) -> Vec<Term> {
    assert!(schema.width < usize::BITS as usize);
    let parity = query % 2;
    let mut out = vec![];
    for word in 0..1usize << schema.width {
        let bits = (0..schema.width)
            .map(|i| (word >> i) & 1)
            .collect::<Vec<_>>();
        let accept = match schema.family {
            "all" | "overlap" => true,
            "equal" => bits
                .iter()
                .all(|&b| b == bits.first().copied().unwrap_or(0)),
            "selective" => bits
                .iter()
                .take(bits.len().saturating_sub(1))
                .all(|&b| b == parity),
            "empty" => bits.is_empty(),
            "redundant" => bits.iter().all(|&b| b == parity),
            _ => panic!("unregistered family"),
        };
        if accept {
            out.push(bits.into_iter().rev().fold(atom("nil"), |tail, b| {
                t("cons", [atom(if b == 0 { "a" } else { "b" }), tail])
            }));
        }
    }
    out.sort();
    out
}
