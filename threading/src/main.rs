use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::rc::Rc;


fn main() {
    let th1 = thread::spawn( || {
        for i in 1..10 {
            println!("number {i} from spawn thread");
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("number {i} in main");
        thread::sleep(Duration::from_millis(200));
    }

    th1.join().unwrap();


    // channels like a go
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("val is {val}");
        tx.send(val).unwrap();
    });
   
    let received = rx.recv().unwrap();
    println!("Got: {received}");

    {   

        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
    
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });


        thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        });
    
        for received in rx {
            println!("Got: {received}");
        }
    }

    // mutex 
    {
        // simple
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {m:?}");
        
        // with threads
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
}
