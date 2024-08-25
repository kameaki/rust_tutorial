use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let num_list = vec![1,2,3];

    let largest = largest(&num_list);
    println!("this is {}", largest);

    let char_list = vec!["a","b","c"];

    let largest = largest2(&char_list);
    println!("this is {}", largest);

    let values = Point {x: 5, y: 4.0};
    println!("values.x = {}", values.x());

    // ジェネリクス
    let p1 = Point {x: 5, y: 4.0};
    let p2 = Point {x: "Hello", y: "c"};
    let p3 = p1.mixup(p2);
    println!("Point = {}, {}", p3.x, p3.y);

    // トレイト
    let value = Text {
        content: String::from("content"),
        name: String::from("hoge"),
    };
    println!("this is {}", value.summarize());
    // トレイトのデフォルトの挙動
    println!("this is {}", value.talk());

    notifiy(&value);
    notifiy2(&value);

    let val = return_summary();
    println!("{}",val.summarize());

    // ライフタイム
    let str1 = String::from("adbcd");
    let str2 = "xyz";
    let result = longest(str1.as_str(), str2);
    println!("{}",result);
}

// ライフタイム
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// トレイト
pub trait Summary {
    fn summarize(&self) -> String;

    fn talk(&self) -> String {
        String::from("READ ME")
    }
}

pub struct Text {
    pub content: String,
    pub name: String,
}

impl Summary for Text {
    fn summarize(&self) -> String {
        format!("{}, {}", self.content, self.name)
    }
}

// トレイトを実装している型を受け付ける関数
pub fn notifiy(item: &impl Summary) {
    println!("ok is {}", item.summarize());
}
pub fn notifiy2<T: Summary>(item: &T) {
    println!("same {}", item.summarize());
}

// 複数のトレイトを実装している必要がある
pub fn notify<T: Summary + Display>(item: &T) {
    println!("twice {}", item.summarize());
}

// whereでトレイトの記載が可能
pub fn notify2<T>(item: &T) 
    where T: Summary + Display {
    println!("twice {}", item.summarize());
}

// トレイトを実装した型を返却する
fn return_summary() -> impl Summary {
    Text {
        content: String::from("content2"),
        name: String::from("hoge2"),
    }
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// ジェネリクスとして、TypeのTを使用
// >を使うのはPartialOrdトレイトを実装している型のみ
// list[0]利用できるのは、Copyトレイトを実装している型のみ
fn largest2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V,W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

