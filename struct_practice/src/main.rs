fn main() {
    println!("Hello, world!");

    // インスタンス生成
    let mut user1 = User {
        user: String::from("hoge"),
        old: 1,
    };

    // 変更
    user1.user = String::from("hoge2");
    user1.old = 2;

    let name = String::from("hoge3");
    let _user2 = build_user(name);
}

struct User {
    // フィールド
    user: String,
    old: u64,
}

fn build_user(user: String) -> User {
    // User {
    //     user: user,
    //     old: 1,
    // }

    User {
        user,
        old: 1,
    }
}
 