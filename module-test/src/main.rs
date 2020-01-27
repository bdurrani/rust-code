mod constants;
mod foo;

mod one {
    pub fn test1() {}
}

fn main() {
    println!("Hello, world! {}", constants::CONSTANT_A);
    one::test1();
    foo::test_foo();
}
