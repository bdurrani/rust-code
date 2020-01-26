pub mod lib1;
use super::constants;

pub struct TestLib {
    item: u8,
}

impl TestLib {
    pub fn new() -> TestLib {
        println!("{}", constants::TEST);
        TestLib { item: 5 }
    }
}
