//! Run with `cargo run -p chr-reference --example explore -- sk-identity`.
use chr_programs::{arithmetic, sk, typing, unary};
use chr_reference::Search;
use chr_syntax::{Query, Var, atom, c, t, v};

fn main() {
    if let Err(message) = run() {
        eprintln!("{message}");
        std::process::exit(2);
    }
}
fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let name = args.next().unwrap_or_else(|| "add".into());
    if name == "--help" {
        println!(
            "explore [add|sub|sk-identity|sk-duplication|sk-synthesize|infer|type-synthesize] [--steps N] [--answers N]"
        );
        return Ok(());
    }
    let mut steps = 10_000usize;
    let mut answer_limit = 10usize;
    while let Some(flag) = args.next() {
        let value: usize = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?
            .parse()
            .map_err(|_| format!("invalid count for {flag}"))?;
        match flag.as_str() {
            "--steps" => steps = value,
            "--answers" => answer_limit = value,
            _ => return Err(format!("unknown option: {flag}")),
        }
    }
    let app = |f, x| t("a", [f, x]);
    let x = t("c", [atom("z")]);
    let y = t("c", [unary(1)]);
    let identity = app(app(atom("s"), atom("k")), atom("k"));
    let duplicate = app(app(atom("s"), atom("s")), app(atom("s"), atom("k")));
    let (rules, constraints, output) = match name.as_str() {
        "add" => (
            arithmetic(),
            vec![c("add", [unary(2), unary(3), v(0)])],
            "sum",
        ),
        "sub" => (
            arithmetic(),
            vec![c("sub", [unary(5), unary(2), v(0)])],
            "difference",
        ),
        "sk-identity" => (
            sk(),
            vec![c("eval", [t("p", [app(identity, x), atom("nil")]), v(0)])],
            "result",
        ),
        "sk-duplication" => (
            sk(),
            vec![c(
                "eval",
                [t("p", [app(app(duplicate, x), y), atom("nil")]), v(0)],
            )],
            "result",
        ),
        "sk-synthesize" => (
            sk(),
            vec![
                c("no_c", [v(0)]),
                c(
                    "eval",
                    [
                        t("p", [app(app(v(0), x.clone()), y.clone()), atom("nil")]),
                        app(app(x, y.clone()), y),
                    ],
                ),
            ],
            "program",
        ),
        "infer" => (typing(), vec![c("infer", [identity, v(0)])], "type"),
        "type-synthesize" => (
            typing(),
            vec![c(
                "infer",
                [
                    v(0),
                    t("fun", [atom("u"), t("fun", [atom("v"), atom("u")])]),
                ],
            )],
            "program",
        ),
        _ => return Err(format!("unknown demo {name}; use --help")),
    };
    let mut search = Search::new(
        rules,
        Query {
            constraints,
            outputs: vec![(output.into(), Var(0))],
        },
    )
    .map_err(|e| format!("invalid program: {e:?}"))?;
    let mut returned = 0;
    let mut exhausted = false;
    let mut used = 0;
    // Single transition requests avoid consuming answers beyond the display limit.
    while used < steps && returned < answer_limit && !exhausted {
        let batch = search.advance(1);
        used += 1;
        exhausted = batch.exhausted;
        for answer in batch.answers {
            returned += 1;
            let outputs = answer
                .outputs
                .iter()
                .map(|(name, term)| format!("{name} = {}", display_term(term)))
                .collect::<Vec<_>>()
                .join(", ");
            let residual = answer
                .residual
                .iter()
                .map(|constraint| display_term(&t(&constraint.name, constraint.args.clone())))
                .collect::<Vec<_>>()
                .join(", ");
            if residual.is_empty() {
                println!("{returned}. {outputs}");
            } else {
                println!("{returned}. {outputs} {{ {residual} }}");
            }
        }
    }
    if exhausted {
        println!("Search exhausted.");
    } else if returned >= answer_limit {
        println!("Answer limit reached; search remains open.");
    } else {
        println!(
            "Step budget exhausted; search remains open. No conclusion about further answers."
        );
    }
    println!(
        "Pending alternatives: {}; stats: {:?}",
        search.pending_alternatives(),
        search.stats()
    );
    Ok(())
}

fn display_term(term: &chr_syntax::Term) -> String {
    match term {
        chr_syntax::Term::Var(Var(id)) => format!("${id}"),
        chr_syntax::Term::App(name, args) if args.is_empty() => name.clone(),
        chr_syntax::Term::App(name, args) => format!(
            "{name}({})",
            args.iter().map(display_term).collect::<Vec<_>>().join(", ")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn terms_display_constructors_and_logical_variables() {
        assert_eq!(display_term(&t("a", [atom("k"), v(2)])), "a(k, $2)");
    }
}
