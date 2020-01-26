mod constants;
mod lib;

fn main() {
    println!("Hello, world! {}", constants::TEST);
    let x = lib::TestLib::new();
    lib::lib1::test_lib1();

}

mod one {
    pub fn test1() {}
}
