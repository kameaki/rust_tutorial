use std::io;
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    let v = vec![1,2,3];

    // v[99];

    let f = File::open("a.txt");
    let f = match f {
        Ok(file) => file,
        // マッチガード、条件に合致しない場合は、次のアームに移る
        // refはムーブされないように使っている
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("a.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("can not create {:?}", e);
                },
            }
        },
        Err(error) => {
            panic!("error is {:?}", error);
        }
    };

    // Errならpanicをしてくれる
    // let f = File::open("b.txt").unwrap();
    // エラーメッセージーをカスタマイズできる
    // let f = File::open("b.txt").expect("okokokok");

    let f = read_file();
    println!("{:?}", f);
}

fn read_file() -> Result<String, io::Error> {
    // let f = File::open("a.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    let mut s = String::new();
    File::open("a.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
