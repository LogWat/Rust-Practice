fn main() {

    // strとStringの変換
    let s1: String = String::from("Very Sleepy!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
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

    for i in &mypets {
        println!("{:?}", i);
    }

}