use super::constants;

pub fn test_foo() {
    println!("test_foo");
    bar::test_bar();
}

mod bar {

    pub fn test_bar() {
        println!("bar {}", super::constants::CONSTANT_A);
        test_bar_internal();
    }

    fn test_bar_internal() {
        println!("test_bar_internal");
    }
}
