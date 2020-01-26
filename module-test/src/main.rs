pub mod constants;
mod lib;

fn main() {
    println!("Hello, world! {}", constants::TEST);
    let x = lib::TestLib::new();
}

mod one {
    pub fn test1() {}
}
