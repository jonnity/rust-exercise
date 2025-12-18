// シンプルスレッド
// use std::sync::mpsc;
// use std::{thread, time};

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         thread::sleep(time::Duration::from_millis(5000));
//         tx.send(val).unwrap();
//     });

//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// デッドロック
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::{thread, time};

fn main() {
    let a = Arc::new(Mutex::new("a"));
    let b = Arc::new(Mutex::new("b"));
    let deadlock_detected = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let deadlock_flag = Arc::clone(&deadlock_detected);
        let handle = thread::spawn(move || {
            println!("Thread 1: aの取得を試みる");
            let mut str_a = a.lock().unwrap();
            *str_a = "a1";

            println!("Thread 1: 1秒待機");

            thread::sleep(time::Duration::from_secs(1));

            println!("Thread 1: bの取得を試みる");
            let start = time::Instant::now();
            loop {
                if deadlock_flag.load(Ordering::Relaxed) {
                    println!("Thread 1: 他のスレッドがデッドロックを検出、終了します");
                    return;
                }

                match b.try_lock() {
                    Ok(mut str_b) => {
                        *str_b = "b1";
                    }
                    Err(_) => {
                        if start.elapsed() > time::Duration::from_secs(2) {
                            eprintln!("Thread 1: デッドロックを検出しました！");
                            deadlock_flag.store(true, Ordering::Relaxed);
                            return;
                        }
                        thread::sleep(time::Duration::from_millis(100));
                    }
                }
            }
        });
        handles.push(handle);
    }

    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let deadlock_flag = Arc::clone(&deadlock_detected);
        let handle = thread::spawn(move || {
            println!("Thread 2: bの取得を試みる");
            let mut str_b = b.lock().unwrap();
            *str_b = "b2";

            println!("Thread 2: すぐにaの取得を試みる");
            let start = time::Instant::now();
            loop {
                if deadlock_flag.load(Ordering::Relaxed) {
                    println!("Thread 2: 他のスレッドがデッドロックを検出、終了します");
                    return;
                }

                match a.try_lock() {
                    Ok(mut str_a) => {
                        *str_a = "a2";
                    }
                    Err(_) => {
                        if start.elapsed() > time::Duration::from_secs(2) {
                            eprintln!("Thread 2: デッドロックを検出しました！");
                            deadlock_flag.store(true, Ordering::Relaxed);
                            return;
                        }
                        thread::sleep(time::Duration::from_millis(100));
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // デッドロックが検出されたかチェック
    if deadlock_detected.load(Ordering::Relaxed) {
        eprintln!("デッドロックが検出されたため、プログラムを異常終了します");
        std::process::exit(1);
    }

    println!(
        "最終結果: a={}, b={}",
        *a.lock().unwrap(),
        *b.lock().unwrap()
    );
}
