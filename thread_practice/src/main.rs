use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    println!("Hello, world!");

    // スレッド
    // mainスレッドが終了すると、新規スレッドは強制終了する
    // 動作順序の保証はない
    thread::spawn(||{
        for i in 1..10 {
            println!("thread is {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main is {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // スレッドの生存期間が不明なため、moveを使用すること
    let v = vec![1,2,3];
    let handle = thread::spawn(move ||{
        println!("{:?}", v);
        for i in 1..10 {
            println!("wait thread is {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main is {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // スレッドの終了を待つ
    handle.join().unwrap();

    // チャンネル
    // txが転送側
    // rxが受信側
    let (tx, rx) = mpsc::channel();

    thread::spawn(move ||{
        let val = String::from("hi");
        // sendはResult型を返す
        // エラーの場合はpanicさせるようにunwrapしている
        tx.send(val).unwrap();

        // valはmoveしているのでエラーが発生する
        // println!("val is {}", val);
    });

    // 値がチャンネルを流れてくるまで、メインスレッドをブロックする
    // try_recvはメインスレッドをブロックしないため、loop処理を流して待つことができる
    let received = rx.recv().unwrap();
    println!("this is {}", received);

    // 複数の転送機を使用する
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![1,2,3,4];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![5,6,7,8];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for value in rx {
        println!("this is {}", value);
    }

    // ミューテックス
    let m = Mutex::new(5);
    {
        // lockでデータアクセスを試みる
        // 現在のスレッドをブロックするので注意
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    // スコープを抜けると、ロックを解除してくれる
    println!("{:?}", m);

    // let counter = Mutex::new(0);
    // Arcを使うことで、複数のスレッド間で安全に共有できる参照カウントを作成できる
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("result {}", *counter.lock().unwrap());
}
