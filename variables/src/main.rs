fn main() {
    // 定数
    const TEISU: u8 = 1;
    println!("{}", TEISU);

    // シャドーウィング
    let x = 3;

    let x = x + 3;

    {
        let x = x * 2;
        println!("innner {}", x);
    }

    // 上記の括弧内のシャドーウィングは継続しない
    println!("out {}", x);

    // シャドーウィングは同じ変数名で型が違う場合に使う
    // let space = "";
    let space = 1;
    println!("{}",space);

    // Rustは静的型付け言語、コンパイル時に型は確定している
    // 型はデータ型のどれかに当てはまる

    // データ型
    // スカラー型
    // 複合型

    // スカラー型
    // 整数
    // おすすめはi32型
    // i8は符号付き、u8は符号なし
    // i8は-2^7から2^7 - 1
    // u8は0から2^8 - 1
    // i8、i16、i32、i64まである
    // isize、usizeはシステムのアーキテクチャに従う
    // 浮動小数点
    // おすすめはf64
    // f32とf64がある
    // char型
    // シングルクォーテーション
    let c = 'z';
    println!("{}",c);

    // 複合型
    // タプル
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}, {}, {}",x, y, z);
    println!("{}",tup.0);
    // 配列
    // 固定長なので注意
    let a = [1,2,3,4,5];
    println!("{}",a[0]);

    let x = another_func(3, 'a');
    println!("{}",x);

    // セミコロンを使わない式
    let y = {
        let x = 3;
        x + 3
    };
    println!("{}",y);

    // if
    let num = if true { 5 } else {6};
    println!("{}",num);

    // while
    let mut num = 10;
    while num != 0 {
        println!("{}",num);
        num -= 1;
    }
    println!("ok");

    // for
    let a = [1,2,3];
    for e in a {
        println!("{}",e);
    }
}

// 関数
fn another_func(x: i32, y: char) -> i32 {
    println!("another {}{}", x, y);
    x + 1000
}