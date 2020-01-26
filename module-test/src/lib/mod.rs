use self::lib1;

pub struct TestLib {
    item: u8,
}

impl TestLib {
    pub fn new() -> TestLib {
        TestLib { item: 5 }
    }
}
