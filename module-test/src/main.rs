mod constants;
mod foo {
    pub fn test_foo() {
        println!("test_foo");
        bar::test_bar();
    }

    mod bar {
        use super::super::constants;
        pub fn test_bar() {
            println!("bar {}", constants::CONSTANT_A);
            test_bar_internal();
        }

        fn test_bar_internal() {
            println!("test_bar_internal");
        }
    }
}

mod one {
    pub fn test1() {}
}

fn main() {
    println!("Hello, world! {}", constants::CONSTANT_A);
    one::test1();
    foo::test_foo();
}