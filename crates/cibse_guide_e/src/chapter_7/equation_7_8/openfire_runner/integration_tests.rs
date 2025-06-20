use framework::method::test::Test;

pub mod test_1;

pub fn tests() -> Vec<Test> {
    vec![test_1::test()]
}
