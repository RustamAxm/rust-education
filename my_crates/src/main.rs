
mod module1;
mod module2;
mod new_import;

use new_import::new_imp::*;

fn main() {
    println!("in main");
    module1::module1::module1_print();
    module2::module2::module2_print();
    print_from_new();
}
