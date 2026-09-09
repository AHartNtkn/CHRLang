pub fn cases() -> Vec<chr_cases::Case> {
    let mut cases = chr_cases::registry();
    for k in [0, 1, 2, 4, 8] {
        for w in [0, 1, 8, 64] {
            for noise in [0, 16] {
                let mut case = chr_cases::carry_case(k, w, noise);
                case.id = format!("duplicate-k{k}-w{w}-n{noise}");
                case.rules[0].body = chr_syntax::or(
                    chr_syntax::eq(chr_syntax::v(0), chr_syntax::atom("a")),
                    chr_syntax::eq(chr_syntax::v(0), chr_syntax::atom("a")),
                );
                // The source generator's first answer has a in every output position.
                // Both explicit arms now bind a; raw derivation count stays 2^k.
                case.expected.truncate(1);
                case.budget = 1_000_000;
                cases.push(case);
            }
        }
    }
    cases
}
