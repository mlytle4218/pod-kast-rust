use 


#[derive(Debug)]
struct TestContext {
    test: bool,
    _bob: String
}

impl TestContext {
    fn new() -> TestContext {
        TestContext {
            test: true,
            _bob: String::from("hiya")
        }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        println!("clean up resources");
    }
}

#[test]
fn try_it() {
    let _ctx = TestContext::new();
    assert!(_ctx.test);
    assert_eq!("hiya", _ctx._bob);
}
