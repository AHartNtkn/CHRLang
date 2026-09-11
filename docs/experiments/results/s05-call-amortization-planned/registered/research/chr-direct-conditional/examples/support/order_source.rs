//! Source-level operand-arrival controls alongside the established stream schema.
#[path = "../../../chr-reuse/examples/support/stream_source.rs"]
mod stream;
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    pub family: &'static str,
    pub resource: bool,
    pub fail_tail: bool,
    pub work: usize,
    pub payload: usize,
}
fn unary(name: &str, end: &str, n: usize) -> Term {
    (0..n).fold(atom(end), |tail, _| t(name, [tail]))
}
impl Schema {
    fn arrival(self) -> bool {
        matches!(self.family, "oldest-first" | "newest-first")
    }
    fn stream(self) -> stream::Schema {
        stream::Schema {
            family: self.family,
            resource: self.resource,
            fail_tail: self.fail_tail,
            work: self.work,
            payload: self.payload,
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        if !self.arrival() {
            return self.stream().rules();
        }
        let mut rules = vec![
            Rule::simplify(
                "coin",
                [c("coin", [v(0)])],
                or(eq(v(0), atom("t")), eq(v(0), atom("f"))),
            ),
            Rule::simplify(
                "check-true",
                [c("check", [t("cons", [atom("t"), v(0)]), v(1), v(2), v(3)])],
                c("wait", [v(1), v(0), v(1), v(2), v(3)]).into(),
            ),
            Rule::simplify(
                "check-false",
                [c("check", [t("cons", [atom("f"), v(0)]), v(1), v(2), v(3)])],
                Goal::Fail,
            ),
            Rule::simplify(
                "wait-step",
                [c("wait", [t("s", [v(0)]), v(1), v(2), v(3), v(4)])],
                c("wait", [v(0), v(1), v(2), v(3), v(4)]).into(),
            ),
            Rule::simplify(
                "wait-done",
                [c("wait", [atom("z"), v(0), v(1), v(2), v(3)])],
                c("check", [v(0), v(1), v(2), v(3)]).into(),
            ),
            Rule::simplify(
                "check-done",
                [c("check", [atom("nil"), v(0), v(1), v(2)])],
                if self.fail_tail {
                    Goal::Fail
                } else {
                    eq(v(2), t("item", [atom("all_true"), atom("all_true"), v(1)]))
                },
            ),
        ];
        if self.resource {
            rules.push(Rule::simplify(
                "finish",
                [
                    c("finish", [t("item", [v(0), v(1), v(2)]), v(3)]),
                    c("token", []),
                ],
                eq(v(3), t("item", [v(0), v(1), v(2)])),
            ));
        }
        rules
    }
    pub fn query(self, n: usize, reverse: bool) -> Query {
        if !self.arrival() {
            return self.stream().query(n, reverse);
        }
        // Preserve coin arrival; reverse changes only inert query tags' position.
        let mut constraints: Vec<_> = (0..n).map(|i| c("coin", [v(100 + i as u64)])).collect();
        let mut ids: Vec<_> = (0..n).collect();
        if self.family == "newest-first" {
            ids.reverse();
        }
        let list = ids
            .into_iter()
            .rev()
            .fold(atom("nil"), |tail, i| t("cons", [v(100 + i as u64), tail]));
        constraints.push(c(
            "check",
            [
                list,
                unary("s", "z", self.work),
                unary("data", "end", self.payload),
                v(10000),
            ],
        ));
        if self.resource {
            constraints.extend([c("finish", [v(10000), v(10001)]), c("token", [])]);
        }
        // The tag varies with the query shape without changing birth ordering.
        let tag = c("tag", [atom(if reverse { "second" } else { "first" })]);
        if reverse {
            constraints.insert(0, tag);
        } else {
            constraints.push(tag);
        }
        let out = Var(if self.resource { 10001 } else { 10000 });
        Query {
            constraints,
            outputs: vec![("result".into(), out), ("again".into(), out)],
        }
    }
    pub fn expected(self, n: usize) -> Vec<Answer> {
        if !self.arrival() {
            return self.stream().expected(n);
        }
        if self.fail_tail {
            return vec![];
        }
        let value = t(
            "item",
            [
                atom("all_true"),
                atom("all_true"),
                unary("data", "end", self.payload),
            ],
        );
        vec![Answer {
            outputs: vec![("result".into(), value.clone()), ("again".into(), value)],
            residual: vec![c("tag", [atom("first")])],
        }]
    }
    pub fn expected_query(self, n: usize, reverse: bool) -> Vec<Answer> {
        let mut answers = self.expected(n);
        if self.arrival() && reverse {
            for a in &mut answers {
                a.residual = vec![c("tag", [atom("second")])];
            }
        }
        answers
    }
}
