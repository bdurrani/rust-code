use crate::constants;

pub fn test_bar() {
    println!("bar {}", constants::CONSTANT_A);
    test_bar_internal();
}

fn test_bar_internal() {
    println!("test_bar_internal");
}
