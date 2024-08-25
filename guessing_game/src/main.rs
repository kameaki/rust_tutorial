// stdは標準ライブラリ
// ioは入出力ライブラリ
use std::io;

use rand::Rng;
// 定義しなくても使えるライブラリはpreludeプレリュード
// cargo doc --openでローカルのwebのドキュメントを生成してくれる

fn main() {
    println!("Guess the number");

    // 下限値は含むが、上限値は含まない
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret: {}", secret_number);
    
    loop {
        println!("Please input");
        // mutをつけて可変変数
        // newはString型の関連関数(関連関数はある型に対して実行される関数のこと)
        let mut guess = String::new();
        // #は参照を表す
        // &mutで可変な参照を表す
        // read_lineはメソッドの戻り値のio::Result型はexpectメソッドを持つ
        // Result型は列挙型でOkとErrという列挙子を持つ
        // expectメソッドはOkの場合は、正常な値を、Errの場合はクラッシュさせ、メッセージを出力する
        io::stdin().read_line(&mut guess).expect("Failed");
    
        // 同じ変数名の使用が可能。シャドーウィング
        // u32の型注釈は、parseで、型が何になるかが確定していないのでつける必要があり
        // match式でexpectメソッドをスマートにすることが可能
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // {}はプレースホルダー
        println!("You guess: {}", guess);
    
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("match");
                break;
            },
            std::cmp::Ordering::Greater => println!("big"),
            std::cmp::Ordering::Less => println!("small"),
        }
    }
}

// cargoビルドで作成してきたものはバイナリクレート
// randはライブラリクレート、依存関係に加えることで使用できるようになる
// 依存関係を更新すると、Crates.ioからrandに必要なデータをDLし、Cargo.lockを作成する
// Cargo.lockを無視する場合は、cargo update
