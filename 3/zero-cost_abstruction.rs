// ゼロコスト抽象化 : trait, dyn

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

fn main() {
    let dove = Dove {};
    let duck = Duck {};
    // 異なる型で異なる処理
    dove.tweet();
    duck.tweet();
    // 異なる型で同じ処理(trait)
    dove.tweet_twice();
    duck.tweet_twice();
    dove.shout();
    duck.shout();

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }
}