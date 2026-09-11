#[cfg(feature="head-dispatch")]
include!("support/finite_lifecycle.rs");
#[cfg(not(feature="head-dispatch"))]
fn main(){panic!("finite lifecycle requires head-dispatch");}
