mod macros;

use macros::{greet, vectorize};

fn main() {

    // @iterators 
    // Using iterators
    // iterators()
    // simulate()
    
    // @macros
    greet!(String::from("Nehemie"));
    let nums = vectorize!(16, f32);
    assert_eq!(nums.len(), 16);
    println!("NUMS {nums:?}")
}
