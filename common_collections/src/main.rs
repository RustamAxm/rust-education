
use std::collections::HashMap;


fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    for el in 1..10 {
        vec.push(el);
    }
    println!("vector done");

//    let val_not_ex = &vec[100];
//    let val_not_ex2 = vec.get(100);
    let mut s = String::new();
    s.push('v');

    let s1 = "second ".to_string();
    // without ownership 
    let mut s3 = String::from("foo");
    let s2 = "bar";
    s3.push_str(s2);
    println!("s3 = {s3}");

    // + operator 
    let s4 = String::from("first");
    let s5 = String::from("second");
    let s6 = s4 + &s5;
    println!("{s6}");

    // iterating 
    for ch in "hihihih".chars() {
        println!("ch {ch}");
    }
    // hash map 
    //
    let mut scores = HashMap::new();
    scores.insert(String::from("ten"), 10);
    
    scores.insert(String::from("nine"), 9);

    let name = String::from("nine");
    let score = scores.get(&name).copied().unwrap_or(0);
    println!("{score}");

    for (key, val) in &scores {
        println!("{key} {val}");
    }

    scores.entry(String::from("nine")).or_insert(11);
    scores.entry(String::from("tw")).or_insert(12);

    for (key, val) in &scores {
        println!("{key} {val}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
     
     
}
