

struct Ints {
    first: u32, 
    second: u64,
    is_valid: bool,
}

struct Color(u32, u32, u32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

fn main() {
    let mut int_ = Ints {
        first: 32,
        second: 21, 
        is_valid: true,
    };
    println!("{}", int_.is_valid);

    int_.second = 45;
    println!("{}", int_.second);

    let col = Color(0, 3, 4);
    println!("{}", col.1);

    let mut rect = Rectangle {
        width: 32,
        height: 23,
    };
    println!("area = {}", area(&rect));

    println!("rect = {rect:#?}");
    dbg!(&rect);
}
