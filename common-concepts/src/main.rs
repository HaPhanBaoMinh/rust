use std::collections::btree_set::Difference;

fn main() {
    // Mut
    let mut x = 5;
    println!("The values of x is {}", x);

    x = 10;
    println!("The values of x is {}", x);

    let y = 10;
    println!("The values of y is {}", y);

    let y = "test";
    println!("The values of y is {}", y);

    const COUNT: u32 = 1000;
    println!("The values of Count is {}", COUNT);

    // Datatype
    // Integers
    let a = 98_222; // Decimal 
    let b = 0xff; // Hex 
    let c = 0o77; //Octal 
    let d = 0b1111_0000; // Binary 
    let e = b'A'; // Byte
    let f = 255;

    // Floating-point numbers
    let f = 2.0;
    let g: f32 = 3.0;

    // addition
    let sum = 5 + 10;
    // ...

    // Booleans
    let c = true;
    let f: bool = false;

    // Character
    let z = "z";

    // Compound Types
    let tup = ("String", 12);
    let (channel, number) = tup;
    let sub_count = tup.1;
    println!("{}", sub_count);

    let err_code = [200, 400, 500];
    let not_found = err_code[1];
    println!("{}", not_found);

    // Function
    my_function(1, "test");

    // Control Flow
    let number = 5;

    if number > 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("last condition was true");
    }

    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        }

        println!("counter is {}", counter);
    }

    let a = [10, 20, 40, 50, 80];

    for e in a.iter() {
        println!("the value is: {}", e);
    }

    for number in (1..5) {
        println!("the number is: {}", number);
    }

    // Line comment
    /*
    :bin
    */
}

fn my_function(x: i32, y: &str) -> i32 {
    println!("Annother function. {} - {}", x, y);

    return x;
}
