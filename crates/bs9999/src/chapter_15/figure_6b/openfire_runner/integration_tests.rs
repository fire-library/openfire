use framework::method::test::Test;

pub mod test_1;
pub mod test_2;
pub mod test_3;
pub mod test_4;

pub fn tests() -> Vec<Test> {
    vec![
        test_1::test(),
        test_2::test(),
        test_3::test(),
        test_4::test(),
    ]
}
