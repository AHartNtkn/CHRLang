pub use crate::engines::{Engine, Event};
use chr_syntax::{Query, Rule};
pub const MODES: [&str; 4] = ["conditional", "inferred", "scan", "resumable"];
pub enum Prepared {
    Conditional(chr_direct_conditional::engine::PreparedRuleset),
    Scan(chr_compiled::PreparedRuleset),
    Resumable(std::sync::Arc<chr_relational::contextual_execute::Prepared>),
}
impl Prepared {
    pub fn new(mode: &str, rules: Vec<Rule>) -> Self {
        match mode {
            "conditional" => Self::Conditional(
                chr_direct_conditional::engine::PreparedRuleset::new(rules).unwrap(),
            ),
            "inferred" => Self::Conditional(
                chr_direct_conditional::engine::PreparedRuleset::with_head_contract(
                    rules,
                    None,
                    chr_direct_conditional::engine::HeadAdmission::Optional,
                )
                .unwrap(),
            ),
            "scan" => Self::Scan(chr_compiled::PreparedRuleset::new(rules, None).unwrap()),
            "resumable" => {
                Self::Resumable(chr_relational::contextual_execute::Prepared::new(&rules).unwrap())
            }
            _ => panic!("mode"),
        }
    }
    pub fn start(&self, query: Query) -> Engine {
        match self {
            Self::Conditional(p) => Engine::Conditional(p.start(query).unwrap()),
            Self::Scan(p) => Engine::Compiled(
                p.start_search(
                    query,
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Scan,
                )
                .unwrap(),
            ),
            Self::Resumable(p) => Engine::Contextual(p.start_resumable(&query)),
        }
    }
}
