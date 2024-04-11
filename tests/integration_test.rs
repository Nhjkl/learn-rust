mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, learning_rust::adds_two(2));
}

// NOTE:
// cargo test --test integration_test
//                     👆测试文件名, 只执行/tests/integration_test.rs 里面的测试函数
