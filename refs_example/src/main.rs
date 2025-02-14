fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}
