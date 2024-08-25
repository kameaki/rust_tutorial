pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// テストを並行で実行
// cargo test
// テストを並列ではなく順番に
// cargo test -- --test-threads=1
// println!を表示したい
// cargo test -- --nocapture
// 単発のテストを実行する
// cargo test it_works

// cargo testの時のみビルドされるようにする
#[cfg(test)]
mod tests {
    use super::*;

    // テスト関数であることを示す
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn sample() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("oh!");
    }

    #[test]
    fn can_hold_pattern() {
        let large = Rect {
            width: 8,
            height: 7
        };
        let small = Rect {
            width: 5,
            height: 1,
        };
        assert!(large.can_hold(&small));
    }

    #[test]
    fn can_hold_patter_small() {
        let large = Rect {
            width: 3,
            height: 0
        };
        let small = Rect {
            width: 5,
            height: 1,
        };
        // カスタムメッセージ
        assert!(large.can_hold(&small),"This is message {}", "OK");
    }

    #[test]
    #[should_panic]
    fn panic_test(){
        let large = Rect {
            width: 0,
            height: 0
        };
        large.can_hold(&large);
    }

    #[test]
    #[should_panic(expected = "1")]
    fn panic_expected_test(){
        let large = Rect {
            width: 1,
            height: 0
        };
        large.can_hold(&large);
    }
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        if self.width == 0 {
            panic!("Good is {}", self.width);
        }
        if self.width == 1 {
            panic!("{}", self.width);
        }
        self.width > other.width && self.height > other.height
    }
}
