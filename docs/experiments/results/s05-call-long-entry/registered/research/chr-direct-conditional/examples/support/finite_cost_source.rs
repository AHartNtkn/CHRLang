//! Complete finite sources for lifecycle measurements; no solver implementation.
#[allow(dead_code)]
#[path = "order_source.rs"]
mod arrival;
use chr_syntax::{Answer, Query, Rule, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    pub family: &'static str,
    pub work: usize,
    pub payload: usize,
    pub fail: bool,
}
impl Schema {
    fn base(self) -> arrival::Schema {
        arrival::Schema {
            family: if self.family == "newest" {
                "newest-first"
            } else {
                "oldest-first"
            },
            work: self.work,
            payload: self.payload,
            resource: true,
            fail_tail: self.fail,
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        let mut rules = self.base().rules();
        if self.family == "all" {
            rules[2].body = rules[1].body.clone();
        }
        if self.family == "duplicates" {
            rules[0].body = or(eq(v(0), atom("t")), rules[0].body.clone());
        }
        rules
    }
    pub fn query(self, n: usize, tag: bool) -> Query {
        let mut q = self.base().query(n, tag);
        if self.family == "all" {
            q.constraints.push(c(
                "witness",
                [(0..n)
                    .rev()
                    .fold(atom("nil"), |xs, i| t("cons", [v(100 + i as u64), xs]))],
            ));
        }
        q
    }
    pub fn count(self, n: usize) -> usize {
        if self.fail {
            0
        } else if matches!(self.family, "all" | "duplicates") {
            1 << n
        } else {
            1
        }
    }
    pub fn expected(self, n: usize, tag: bool) -> Vec<Answer> {
        let mut a = self.base().expected_query(n, tag);
        if a.is_empty() {
            return a;
        }
        let first = a.pop().unwrap();
        (0..self.count(n))
            .map(|bits| {
                let mut a = first.clone();
                if self.family == "all" {
                    let list = (0..n).rev().fold(atom("nil"), |xs, i| {
                        t(
                            "cons",
                            [atom(if bits & (1 << i) == 0 { "t" } else { "f" }), xs],
                        )
                    });
                    a.residual.push(c("witness", [list]));
                }
                a
            })
            .collect()
    }
}
