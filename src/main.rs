mod macros;

use core::num;

use macros::{greet, vectorize};

fn main() {
    // Using iterators
    // iterators()
    // simulate()
    greet!(String::from("Nehemie"));
    let nums = vectorize!(16);
    assert_eq!(nums.len(), 16);
    println!("NUMS {nums:?}")
}
