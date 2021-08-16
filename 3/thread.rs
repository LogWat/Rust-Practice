use std::thread;
use std::sync::{Arc, Mutex};

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

    let mut data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone(); // <= Arcの参照カウンタ増加
        handles2.push(thread::spawn(move || {
            // dataへの可変参照を.lock()によって生成
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles2 {
        let _ = handle.join();
    }

    dbg!(data);
}