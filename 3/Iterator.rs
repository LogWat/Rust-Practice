// イテレータによってforが使える
// => 自作した型(ex. struct)にイテレータトレイトを適用すれば
//    自作の型でもforが使える

struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize; // 出力する型の紐づけ

    // next() method
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}

fn main() {
    let it = Iter {
        current: 100,
        max: 110,
    };
    for num in it {
        println!("{}", num);
    }
}