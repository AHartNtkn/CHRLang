#[cfg(feature = "head-dispatch")]
include!("support/order_lifecycle.rs");
#[cfg(not(feature = "head-dispatch"))]
fn main() {
    panic!("order lifecycle requires experiment and head-dispatch features");
}
