struct character {
    name: String,
    level: usize,
    fav: usize,
}

#[derive(Eq, PartialEq)]
struct A(isize);

impl character {
    fn disp_chara(&self) {
        println!("This character name is {}.", self.name);
        println!("Level = {}", self.level);
        println!("Fav Level = {}", self.fav);
    }
    fn level_up(&mut self) {
        self.level += 1;
        println!("[update] This character's level is {}", self.level);
    }
    fn fav_up(&mut self, by: usize) {
        self.fav += by;
        println!("[update] This character's fav level is {}", self.fav);
    }

    // スレッドチェーン
    fn disp_name(&self) -> &Self {
        println!("This character name is {}.", self.name);
        self
    }
    fn disp_level(&self) -> &Self {
        println!("This character's level is {}.", self.level);
        self
    }
}


fn func(code: isize) -> Result<isize, String> {
    println!("code: {}", code);
    Ok(100)
}

// ? 演算子
fn error_handling(result: Result<isize, String>) -> Result<isize, String> {
    let code = result?; // Okならcode=Ok(), Errならreturn
    println!("code: {}", code);
    Ok(100)
}

// バイト数表示
fn my_print(s: Box<[u8]>) {
    println!("{:?}", s);
}

fn abs(number: isize) -> isize {
    if number < 0 {
        return -number;
    }
    number
}

fn main() {

    // strとStringの変換==================================
    let s1: String = String::from("Very Sleepy!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("========");
    println!("{}", s3);

    // 配列, struct======================================
    struct Pets {
        name: String,
        age: u32,
    }

    let mut mypets: [Pets; 2] = [
        Pets {
            name: String::from("Hachi"),
            age: 4,
        },
        Pets {
            name: String::from("Nana"),
            age: 6,
        }
    ];
    mypets[1].age += 1; // inc使えない? 0-indexed

    println!("========");
    for i in &mypets {
        println!("{} : {}", i.name, i.age);
    }
    println!("========");

    // 頻出する標準ライブラリの型==============================
    // ex. Resultでのパターンマッチの例
    let r1: Result<isize, String> = Ok(200);
    match r1 {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    };
    let r2: Result<isize, String> = Ok(200);
    if let Ok(code) = r2 {
        println!("code: {}", code);
    }
    // ネストが深くならないような書き方
    let r3: Result<isize, String> = Err("error".to_string());
    println!("code: {}", r3.unwrap_or(-1)); // -1が出力される

    // .unwrap_or()以外のメソッド
    let r4: Result<isize, String> = r2.and_then(func);
    let r0: Result<isize, String> = Err("error".to_string());
    let r5: Result<isize, String> = r0.and_then(func); // <- 実行されない
    println!("========");

    // Vec
    let mut v1 = vec![0, 1, 2, 3, 4];
    let mut v2 = vec![0; 5]; // 縦行列ではなく5列の要素全て0
    v1.push(5);
    v2.pop();
    for i in &v1 {
        print!("{} ", i);
    }
    for i in &v2 {
        print!("{} ", i)
    }
    println!("");
    println!("========");

    // Box
    let byte_array = [b'H', b'e', b'l', b'l', b'o'];
    my_print(Box::new(byte_array)); // そのまま渡すとエラー
    println!("========");

    //=====================================================
    // if
    let val0 = 1;
    let val1 = if val0 > 0 {
        val0
    } else {
        -val0
    };
    println!("val1 = {}", val1);

    // 繰り返し処理
    let mut count0 = 0;
    let val2 = loop {
        println!("count is {}", count0);
        count0 += 1;
        if count0 == 10 {
            break count0;
        }
    };
    println!("val2 is {}", val2);

    for i in 0..10 { // <= range(10)のような書き方 0..=10もOK
        println!("i is {}", i);
    }
    for i in 1..=10 {
        println!("{}", i)
    }
    
    // match (switch)
    let i = 334;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("OHH MY GOD"),
    }

    // functions **************
    // basic
    println!("{}", abs(-100)); 

    // impl
    let mut t = character {
        name: String::from("irohanihoheto"),
        level: 15,
        fav: 5,
    };
    t.disp_chara();
    t.level_up();
    t.fav_up(2);
    t.disp_chara();
    // メソッドチェーン
    t.disp_name().disp_level();

    println!("{:?}", A(29) == A(28));

    let important_data = "Hayakunetaindesukedo".to_string();
    calc_data(&important_data); // 参照渡し

    let x1 = 100;
    let y1 = &x1; // 不可変の参照渡しはいくらでも可能
    dbg!(y1);
    let mut x2 = 1000;
    let mut y2 = &x2; // 可変の参照渡しは一つだけしかできない
    dbg!(y2);
}

fn calc_data(data: &String) {
    println!("{}", data);
}