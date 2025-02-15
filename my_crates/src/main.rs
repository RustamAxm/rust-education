
mod module1;
mod module2;

fn main() {
    println!("in main");
    module1::module1::module1_print();
    module2::module2::module2_print();
}
