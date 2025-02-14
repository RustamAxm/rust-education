

fn main() {
    let x = 5;
    let y = 6.0;
    println!("{x} {y}");

    // scope 
    {
        let x = 50; 
        println!("in scope x = {x}");
    }
    println!("out scope x = {x}");

    // types
    let a: u8 = 3;
    let b: i32 = -2;
    let c: f32 = 34.9;
    let d: bool = true;
    let f: char = 'c'; 

    println!("{a} {b} {c} {d} {f}");
    
    print_type_of(&a);
    print_type_of(&c);

    // tupple 
    let tup: (u8, f32, bool) = (2, 3.0, true);
    println!("tup 1 2 = {} {}", tup.1, tup.2);

    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("get index 1 = {}", a[1]);

    for el in a {
        println!("{}", el);
    }

    // ret func
    let val = return_some(15);
    println!("ret val = {}", val);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn return_some(a: u32) -> u32 {
    return a + 1;
}
