mod constants;

//use crate::lib::lib_const;
//use lib::lib_const;
mod lib;

fn main() {
    println!("Hello, world! {}", constants::TEST);
//    let x = lib::TestLib::new();
    lib::lib1::test_lib1();
    lib::lib_const::LIB_TEST;

}

mod one {
    pub fn test1() {}
}
