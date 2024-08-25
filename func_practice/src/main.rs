use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let power = 10;
    let num = 6;

    work(power, num);

    let x = 4;
    // クロージャー
    let equal = |z| z == x;
    println!("{}", x);
    let y = 4;
    assert!(equal(y));

    let x = vec![1,2,3];
    let equal = move |z| z == x;
    // 所有権が奪われているため、使えない
    // println!("{}", x);
    let y = vec![1,2,3];
    assert!(equal(y));

    // イテレータ
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();
    assert_eq!(sum, 6);

    let v1 = vec![1,2,3];
    let v1_vec: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v1_vec, [2,3,4]);

    let v1 = vec![1,2,3];
    let v1_vec: Vec<_> = v1.into_iter().filter(|s| *s == 2).collect();
    assert_eq!(v1_vec, [2]);

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// fn simulate(power: u32) -> u32 {
//     println!("slow");
//     thread::sleep(Duration::from_secs(2));
//     power
// }

fn work(power: u32, num: u32) {
    // let calc = simulate(power);
    // let closer = |number| {
    //     println!("slow");
    //     thread::sleep(Duration::from_secs(2));
    //     number
    // };
    let mut cache = Cache::new(
        |number| {
            println!("slow");
            thread::sleep(Duration::from_secs(2));
            number
        }
    );

    if power < 25 {
        // simulate(power);
        // simulate(power);
        // println!("{}",calc);
        // println!("{}",calc);
        // closer(power);
        // closer(power);
        cache.value(power);
        cache.value(power);
    } else {
        if num == 3 {
            println!("ok");
        } else {
            // simulate(power);
            // println!("{}",calc);
            // closer(power);
            cache.value(power);
        }
    }
}

struct Cache<T> where T: Fn(u32) -> u32 {
    calc: T,
    value: Option<u32>,
}

impl<T> Cache<T> where T: Fn(u32) -> u32 {
    fn new(calc: T) -> Cache<T> {
        Cache {
            calc,
            value: None,
        }
    }

    // 一度値を追加すると、変更できないので、ハッシュマップを使用する
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count : 0
        }
    }
}

// Iteratorを独自に実装する
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
