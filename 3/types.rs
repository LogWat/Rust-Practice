fn main() {

    // strとStringの変換
    let s1: String = String::from("Very Sleepy!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("========");
    println!("{}", s3);

    // 配列, struct
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

    // 頻出する標準ライブラリの型
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
    println!("code: {}", r3.unwrap_or(-1)) // -1が出力される


}