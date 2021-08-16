use std::thread;
use std::rc::Rc;

fn main() {
    let mut handles = Vec::new();
    let mut handles2 = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello, world!: {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    let mut data = Rc::new(Vec![1; 10]);

    for x in 0..10 {
        let data_ref = data.clone(); // <= Rcの参照カウンタ増加
        handles2.push(thread::spawn(move || {
            data_ref[x] += 1;
        }));
    }

    for handle in handles2 {
        let _ = handle.join();
    }

    dbg!(data);
}