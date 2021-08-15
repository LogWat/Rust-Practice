// ゼロコスト抽象化 : trait, dyn
// (マーカトレイト)
// ジェネリクス

trait Tweet {
    
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("AAAAAAAAAAAAAHHHHHHHHHHHH!!!!!!!!!!!!!");
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

// ジェネリクス
fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

fn main() {
    let dove = Dove {};
    let duck = Duck {};
    // 静的ディスパッチ
    // 異なる型で異なる処理
    dove.tweet();
    duck.tweet();
    // 異なる型で同じ処理(trait)
    dove.tweet_twice();
    duck.tweet_twice();
    dove.shout();
    duck.shout();

    // 動的ディスパッチ(dyn)
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet(); // <= Dove型か，Duck型かは実行してみないと分らない
    }

    // ジェネリクス
    let tuple1 = make_tuple(1, 2);
    let tuple2 = make_tuple(323, "Hello~");
    println!("tuple1 = {:?}, tuple2 = {:?}", tuple1, tuple2);
}