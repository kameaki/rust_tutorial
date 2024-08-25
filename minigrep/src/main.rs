extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;


fn main() {
    // let args: Vec<String> = env::args().collect();

    // 0番目は"target/debug/minigrep"
    // let config = parse(&args);
    // let config = Config::new(&args);
    // let config = Config::new(&args).unwrap_or_else(|err| {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        // エラーの出力をターミナルに出力させる
        // cargo run > out.txtには、出力されない
        eprintln!("error is {}", err);
        // 即座にプログラムを終了させ、1を返却する
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("error two is {}", e);
        process::exit(1);
    }
}
