fn main() {
    let v = vec![0; 10];

    for element in &v {
        println!("{}", element);
    }

    let normal = 100;
    let mut changeable = 100;
    changeable += normal;
    println!("{}", changeable);
}