fn main() {
    println!("Hello, world!");

    let width = 30;
    let height = 50;

    println!("this is {}", area(width, height));

    let rect = (30, 50);
    println!("this is {}", area2(rect));

    let rect1 = Rect { width: 30, height: 50 };
    // ムーブするより、不変借用の方が安全
    println!("this is {}", area3(&rect1));
    
    // printlnで表示するために必要
    println!("rect is {:?}", rect1);

    // 構造体のメソッドを利用
    println!("this is {}", rect1.area());

    // 関連関数の呼び出し方。String::fromと同じ
    println!("square is {:?}", Rect::square(32));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(val: (u32, u32)) -> u32 {
    val.0 * val.1
}

fn area3(val: &Rect) -> u32 {
    val.height * val.width
}

// printlnで表示するために必要
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
} 