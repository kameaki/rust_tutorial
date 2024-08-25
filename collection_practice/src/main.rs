use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // ベクタ
    let v : Vec<i32> = Vec::new();
    // ベクタのマクロ Vec<i32>
    let v2 = vec![1,2,3];

    let mut v = Vec::new();
    v.push(5);

    let third : &i32 = &v2[2];
    println!("this is {}", third);
    // getはOptionを返却する
    match v2.get(2) {
        Some(third) => println!("ok"),
        None => println!("no"),
    }

    let mut v = vec![1,2,3];
    for i in &mut v {
        // 参照はずしを行なっている
        *i += 50;
        println!("{}", i);
    }

    // String
    let s1 = String::from("hoge");
    let s2 = String::from("ok");
    // s1がムーブされ、s2は参照を渡している
    let s3 = s1 + &s2;
    println!("this is {}", s3);
    let s4 = String::from("no");

    // ムーブされていない
    let s5 = format!("{}{}",s3,s4);
    println!("this is {}", s3);
    println!("this is {}", s5);


    // ハッシュマップ
    let mut score = HashMap::new();
    score.insert(String::from("ok"), 10);

    let key_name = String::from("ok");
    // valueはOption
    let value = score.get(&key_name);
    println!("{:?}",score);

    // 値の上書き
    score.insert(String::from("ok"), 100);
    println!("{:?}",score);

    // 値がなければ加える
    score.entry(String::from("no")).or_insert(50);
    println!("{:?}",score);
}
