mod front_of_house {
    pub mod hosting {
        pub fn add_wait() {}

        fn seat() {}
    }

    pub mod service {
        pub fn order() {}

        fn serve() {}

        fn payment() {}
    }

    fn cook() {
        super::sorry();
    }

    // structも公開できる
    pub struct Food {
        price: u32,
        pub name: String,
    }

    impl Food {
        pub fn make() -> Food {
            Food {
                price: 32,
                name: String::from("ok"),
            }
        }
    }
}

// useで楽に利用できる
use crate::front_of_house::hosting;
use crate::front_of_house::service as Hoge;
use crate::front_of_house::*;

fn sorry() {}

pub fn eat() {
    // eatとfront_of_houseは同じモジュール内に定義されているため、参照可能
    crate::front_of_house::hosting::add_wait();

    front_of_house::hosting::add_wait();

    let mut food = front_of_house::Food::make();

    food.name = String::from("name");

    println!("this is {}", food.name);

    hosting::add_wait();

    Hoge::order();
}