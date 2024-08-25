use List::{Cons, Nil};
use List2::*;
use List3::*;
use std::rc::{Rc, Weak};
use std::ops::Deref;
use std::cell::RefCell;

fn main() {
    println!("Hello, world!");

    let b = Box::new(5);
    println!("{}",b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, 
        Box::new(Cons(2, 
                Box::new(Cons(3, 
                    Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    // *(y.deref())を内部的には処理している
    assert_eq!(5, *y);

    // main終了後の自動dropより先にdropする
    drop(y);
    println!("early drop");

    // let a = Cons(5,
    //     Box::new(Cons(10,
    //         Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    // 参照カウント
    let a = Rc::new(List2::Cons(5,
        Rc::new(List2::Cons(10,
            Rc::new(List2::Nil)))));
    println!("{}", Rc::strong_count(&a));
    // Rc::cloneは参照カウントを増やすだけ
    let b = Rc::new(List2::Cons(3, Rc::clone(&a)));
    println!("{}", Rc::strong_count(&a));
    {
        let c = Rc::new(List2::Cons(4, Rc::clone(&a)));
        println!("{}", Rc::strong_count(&a));
    }
    println!("{}", Rc::strong_count(&a));

    // 内部可変性
    // 参照カウントは不変参照だったが、可変参照に変更することが可能
    // Rc<RefCell<i32>>を作成している
    let value = Rc::new(RefCell::new(5));
    // Rc::clone(&value)でaにvalueの所有権を共有している
    let a = Rc::new(List3::Cons(
        Rc::clone(&value),
        Rc::new(List3::Nil)
    ));
    let b = List3::Cons(
        Rc::new(RefCell::new(6)),
        Rc::clone(&a)
    );
    let c = List3::Cons(
        Rc::new(RefCell::new(10)),
        Rc::clone(&a)
    );

    *value.borrow_mut() += 10;

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    // 強参照と弱参照
    let leaf = Rc::new(
        Node {
            value: 3,
            child: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        }
    );
    // upgradeでWeakが参照する値を確認できる
    // ただし、ドロップされているかもしれないので、Optionを返却する
    println!("parent is {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(
        Node {
            value: 5,
            child: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        }
    );
    // Rc::downgradeで弱参照を作ることできる
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("parent is {:?}", leaf.parent.borrow().upgrade())
}

enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

#[derive(Debug)]
enum List3 {
    Cons(Rc<RefCell<i32>>, Rc<List3>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // 参照を返却する
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop");
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    // Nodeを他のNodeと共有できるようにRc<T>
    // Nodeの関係を変更できるようにRefCell<T>
    child: RefCell<Vec<Rc<Node>>>,
    // 弱参照を使用する
    // 親を参照はできるが、所有はしない
    parent: RefCell<Weak<Node>>,
}