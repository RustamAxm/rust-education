use std::thread;
use std::time::Duration;


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
}
