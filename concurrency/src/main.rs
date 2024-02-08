use std::thread;
use std::rc::Rc;
use std::time::Duration;
use std::sync::{Mutex, Arc};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
//fn main() {
//    let (tx, rx) = mpsc::channel();
//
//    let tx1 = mpsc::Sender::clone(&tx);
//    thread::spawn(move || {
//        let vals = vec![
//            String::from("hi"),
//            String::from("from"),
//            String::from("the"),
//            String::from("thread"),
//        ];
//
//        for val in vals {
//            tx1.send(val).unwrap();
//            thread::sleep(Duration::from_secs(1));
//        }
//    });
//
//    thread::spawn(move || {
//        // 君のためにもっとメッセージを(more messages for you)
//        let vals = vec![
//            String::from("more"),
//            String::from("messages"),
//            String::from("for"),
//            String::from("you"),
//        ];
//    
//        for val in vals {
//            tx.send(val).unwrap();
//            thread::sleep(Duration::from_secs(1));
//        }
//    });
//
//    for received in rx {
//        println!("Got: {}", received);
//    }
//    
//}

//fn main() {
//    let v = vec![1, 2, 3];
//
//    let handle = thread::spawn(move || {
//        println!("ハンドル内: {:?}", v);
//    });
//    
//    handle.join().unwrap();
//}

//fn main() {
//    let handle = thread::spawn(|| {
//        for i in 1..10 {
//            println!("新しいスレッドから{}番目", i);
//            thread::sleep(Duration::from_millis(1));
//        }
//    });
//
//    for i in 1..5 {
//        println!("メインスレッドから{}番目", i);
//        thread::sleep(Duration::from_millis(1));
//    };
//
//    handle.join().unwrap();
//}