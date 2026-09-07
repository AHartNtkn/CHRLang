use chr_observe::graph::{AnswerView, Stats, TermView, equivalent};
struct View {
    nodes: Vec<N>,
    outputs: Vec<usize>,
    residual: Vec<Vec<usize>>,
}
enum N {
    V(u64),
    F(&'static str, Vec<usize>),
}
impl AnswerView for View {
    type Handle = usize;
    type Variable = u64;
    fn output_count(&self) -> usize {
        self.outputs.len()
    }
    fn output(&self, i: usize) -> (&str, usize) {
        ("out", self.outputs[i])
    }
    fn residual_count(&self) -> usize {
        self.residual.len()
    }
    fn residual_name(&self, _: usize) -> &str {
        "p"
    }
    fn residual_arity(&self, i: usize) -> usize {
        self.residual[i].len()
    }
    fn residual_arg(&self, i: usize, j: usize) -> usize {
        self.residual[i][j]
    }
    fn constructor_child(&self, h: usize, i: usize) -> usize {
        let N::F(_, a) = &self.nodes[h] else {
            panic!("constructor")
        };
        a[i]
    }
    fn resolve(&self, h: usize, _: &mut Stats) -> TermView<'_, usize, u64> {
        match &self.nodes[h] {
            N::V(v) => TermView::Variable(*v),
            N::F(n, a) => TermView::Constructor(n, h, a.len()),
        }
    }
}
#[test]
fn physical_sharing_and_equal_numeric_handles_do_not_decide_denotation() {
    let a = View {
        nodes: vec![N::V(0), N::F("f", vec![0])],
        outputs: vec![1, 1],
        residual: vec![],
    };
    let mut b = View {
        nodes: vec![N::V(9), N::F("f", vec![0]), N::F("f", vec![0])],
        outputs: vec![1, 2],
        residual: vec![],
    };
    assert!(equivalent(&a, &b, &mut Stats::default()));
    b.nodes[2] = N::F("g", vec![0]);
    assert!(!equivalent(&a, &b, &mut Stats::default()));
    b.outputs = vec![1, 1];
    b.nodes[1] = N::F("g", vec![0]);
    assert!(!equivalent(&a, &b, &mut Stats::default()));
}
#[test]
fn rollback_and_joint_aliases_preserve_residual_multisets() {
    let a = View {
        nodes: vec![N::V(0), N::V(1), N::V(2)],
        outputs: vec![0],
        residual: vec![vec![1, 2], vec![0, 1], vec![2, 2]],
    };
    let mut b = View {
        nodes: vec![N::V(8), N::V(9), N::V(10)],
        outputs: vec![0],
        residual: vec![vec![0, 1], vec![2, 2], vec![1, 2]],
    };
    let mut stats = Stats::default();
    assert!(equivalent(&a, &b, &mut stats));
    assert!(stats.backtracks > 0);
    b.residual[1] = vec![1, 1];
    assert!(!equivalent(&a, &b, &mut Stats::default()));
}

#[test]
fn borrowed_tree_control_uses_the_same_joint_mapping_and_backtracking() {
    use chr_observe::graph::TreeView;
    use chr_syntax::{Answer, c, t, v};
    let a = Answer {
        outputs: vec![("out".into(), t("f", [v(0)]))],
        residual: vec![
            c("p", [v(1), v(2)]),
            c("p", [v(0), v(1)]),
            c("p", [v(2), v(2)]),
        ],
    };
    let b = Answer {
        outputs: vec![("out".into(), t("f", [v(9)]))],
        residual: vec![
            c("p", [v(9), v(8)]),
            c("p", [v(7), v(7)]),
            c("p", [v(8), v(7)]),
        ],
    };
    let mut stats = Stats::default();
    assert!(equivalent(&TreeView(&a), &TreeView(&b), &mut stats));
    assert!(stats.backtracks > 0);
    assert_eq!(stats.dereferences, 0);
    assert_eq!(stats.binding_visits, 0);
    let mut bad = b.clone();
    bad.outputs[0].1 = t("f", [v(8)]);
    assert!(!equivalent(
        &TreeView(&a),
        &TreeView(&bad),
        &mut Stats::default()
    ));
}
