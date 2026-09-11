#[path = "support/continuation_source.rs"]
mod source;
use chr_persistent::continuations::{Machine, Step};
use std::collections::VecDeque;
fn main() {
    let schema = source::Schema::new("history", false);
    let (mut machine, cursor) = Machine::new(schema.rules(), schema.query(1, false)).unwrap();
    let mut queue = VecDeque::from([cursor]);
    let mut answers = 0;
    for i in 0..1000 {
        let Some(cursor) = queue.pop_front() else {
            break;
        };
        println!("{i} {:?}", machine.key(&cursor).alpha_live_history());
        match machine.step(cursor) {
            Step::Continue(c) => queue.push_back(c),
            Step::Split(a, b) => {
                queue.push_back(a);
                queue.push_back(b);
            }
            Step::Answer(_) => answers += 1,
            Step::Failed => {}
        }
    }
    assert!(queue.is_empty());
    assert_eq!(answers, schema.answer_count());
}
