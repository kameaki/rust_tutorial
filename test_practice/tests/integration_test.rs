extern crate test_practice;

// 結合テストを実行する
// cargo test --test integration_test
#[test]
fn it_add_two(){
    assert_eq!(4, test_practice::add(2, 2));
}