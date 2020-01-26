pub mod lib1;
use self::lib1::test_lib1;
use super::constants;

pub struct TestLib {
    item: u8,
}

impl TestLib {
    pub fn new() -> TestLib {
        println!("{}", constants::TEST);
        test_lib1();
        TestLib { item: 5 }
    }
}
