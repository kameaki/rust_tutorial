fn main() {
    // ヒープ

    // 文字列型は2種類存在する
    // 文字列リテラル：スタックを使用、不変
    let x = "aaaa";
    // String型：ヒープを使用、可変
    // fromを使用して、文字リテラルからString型に変更＝＞OSにメモリを要求している
    let mut y = String::from(x);
    y.push_str("world");
    println!("{}", y);

    {
        // メモリの確保をOSに要求する
        let _s = String::from("hello");
    }
    // sはスコープを抜ける＝＞メモリが解放される　＝　dropという関数を暗黙的に呼んでいる

    // ムーブ
    // エラーの例
    // let x = String::from("hello");
    // let y = x;
    // println!("{}", x);

    // 参照
     let s1 = String::from("hello");
     let len = calc(&s1);
     println!("{}, {}", s1, len);

     // 可変参照
     let mut s = String::from("hello");
     change(&mut s);
     println!("{}", s);
     // エラ
    //  let r1 = &mut s;
    //  let r2 = &mut s;
    //  println!("{}, {}", r1, r2);

    // 文字列スライス
    let mut s = String::from("hello world");
    let _word = first_word(&s);
    s.clear();
    // first_wordが文字列スライスを利用していなければ、コンパイルエラーが発生してくれない
    // println!("{}", word);

    let my_string = String::from("hello world");
    let _word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // 文字列リテラルは、文字列スライスの型を持っている
    let _word = first_word(&my_string_literal);
}

fn calc(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("ok");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
} 