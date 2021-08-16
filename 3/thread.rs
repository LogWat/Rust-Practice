use std::thread;
use std::sync::{Arc, Mutex, mpsc};

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
    // 共有メモリによるスレッド間データ共有
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

    // メッセージパッシングによるデータ取扱
    let mut handles3 = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        // mainから各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();
        // 各スレッドからmainへのチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles3.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    // 各スレッドにdataの値を送信
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    // 各スレッドからの結果をdataに格納
    for x in 0..10 {
        let _ = rcv_channels[x].recv().unwrap();
    }

    for handle in handles3 {
        let _ = handle.join();
    }

    dbg!(data);
}