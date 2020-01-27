//mod __foo;
mod foo;

fn main() {
    println!("Hello, world!");
    one::test1();
    foo::test_foo();
}

mod one {
    pub fn test1() {}
}
