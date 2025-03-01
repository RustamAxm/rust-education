use oop::AveragedCollection;

fn main() {
    let mut val = AveragedCollection::new();
    val.add(32);
    println!("{}", val.average());
}
