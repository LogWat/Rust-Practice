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

    // Box
    let byte_array = [b'H', b'e', b'l', b'l', b'o'];
    my_print(Box::new(byte_array)); // そのまま渡すとエラー

}