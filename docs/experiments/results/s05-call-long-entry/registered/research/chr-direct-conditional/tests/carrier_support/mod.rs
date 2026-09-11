//! Independent complete observation oracle for rejected carrier admission.
use chr_syntax::{Answer, Query, Term, atom, c, t, v};
pub fn query(pre: usize) -> Query {
    let control = (0..pre).fold(v(42), |rest, _| t("s", [rest]));
    Query {
        constraints: vec![c("start", [control, atom("z")])],
        outputs: vec![],
    }
}
pub fn answer_key(answer: &Answer) -> Option<u8> {
    if !answer.outputs.is_empty() || answer.residual.len() != 1 {
        return None;
    }
    let residual = &answer.residual[0];
    if residual.name != "pre" {
        return None;
    }
    let [
        Term::Var(control),
        post,
        Term::App(name, bits),
        Term::Var(shared),
    ] = residual.args.as_slice()
    else {
        return None;
    };
    if control == shared
        || !matches!(post, Term::App(n,args) if n=="z" && args.is_empty())
        || name != "tuple"
        || bits.len() != 4
    {
        return None;
    }
    let mut key = 0;
    for (i, bit) in bits.iter().enumerate() {
        match bit {
            Term::App(n, args) if n == "a" && args.is_empty() => (),
            Term::App(n, args) if n == "b" && args.is_empty() => key |= 1 << i,
            _ => return None,
        }
    }
    Some(key)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle_rejects_bound_or_identified_unknowns_and_wrong_residuals() {
        let mut a = Answer {
            outputs: vec![],
            residual: vec![c(
                "pre",
                [
                    v(1),
                    atom("z"),
                    t("tuple", [atom("a"), atom("a"), atom("a"), atom("a")]),
                    v(2),
                ],
            )],
        };
        assert_eq!(answer_key(&a), Some(0));
        a.residual[0].args[0] = v(2);
        assert!(answer_key(&a).is_none());
        a.residual[0].args[0] = atom("z");
        assert!(answer_key(&a).is_none());
        a.residual[0].args[0] = v(1);
        a.residual.push(c("extra", []));
        assert!(answer_key(&a).is_none());
    }
}
