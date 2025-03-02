struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("!!!!!!simple if let !!!!");
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
    println!("!!!!while let Conditional Loops!!!!!");
    {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for val in [1, 2, 3] {
                tx.send(val).unwrap();
            }
        });

        while let Ok(value) = rx.recv() {
            println!("{value}");
        }
    }

    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{value} is at index {index}");
        }
    } 
    println!("!!!!!get vals from struct !!!!");
    {
        let p = Point { x: 0, y: 7};
        let Point {x: a, y: b} = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
        println!("a = {a}, b = {b}");
        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }
    println!("!!!!!!!!ignoring values!!!!!!!");
    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}")
            }
        }

        match numbers {
            (first, second, ..) => {
                println!("Some numbers: {first} {second}")
            },
        }
    }
    println!("!!!!! bindings !!!!");
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {id_variable}"),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
    }
}
