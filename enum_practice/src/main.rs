fn main() {
    println!("Hello, world!");

    let four = Address::V4(0);
    let six = Address::V6(String::from("hoge2"));

    route(&four);
    route(&six);

    six.call();

    // Option型について
    let some_num = Some(5);
    let _some_str = Some("hoge");
    let _absent_num: Option<i32> = None;

    // 値を取り出すためにmatchを使用する
    let five = Some(5);
    let _six = plus_one(five);
    let _none =  plus_one(None);

    if let Some(6) = some_num {
        println!("hoge");
    } else {
        println!("error");
    }
}

fn route(_enum_type: &Address) {}

enum Address {
    V4(u8),
    V6(String),
}

impl Address {
    fn call(&self) {
        println!("hoge");
    }
}

// とても使われる例がOption
// enum Option<T> {
//     Some(T),
//     None,
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    // matchでiの値を取得していることがわかる
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}