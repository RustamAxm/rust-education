fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'named_loop: loop {
        println!{"count = {count}"};
        let mut remaining = 10; 

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            } 
            if count == 2 {
                break 'named_loop;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count = {count}");

    // while
    let mut val = 5; 

    while val != 0 {
        println!("{val}");
        val -= 1;
    }
    println!("while end");

    //range based
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for num in (1..=5).rev() {
        println!("rev vals {num}");
    }
}
