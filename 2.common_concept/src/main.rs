fn main() {
    // Mut concept
    let mut x = 5;
    println!("x is {}", x);

    x = 6;
    println!("x is {}", x);

    const CONST_VALUE: i32 = 1000;

    // Data types concept
    /// Integers
    /// Floating-point
    /// Booleans
    /// Character

    /// Compund types
    let tup = ("Hello word", 123);
    let (sayhi, count) = tup;

    let sub_count = tup.1;

    println!("tup.1 {}", tup.0);
    println!("sayhi {}", sayhi);
    println!("count {}", count);

    /// Array
    let arr = [12, 12, 12];
    // let x = arr[3];
    let x = arr[1];
    println!("arr[1] {}", x);

    // Functions
    let sum: i32 = my_func(1, 2);
    println!("Sum is {}", sum);

    // Control flow
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number is {}", number);

    let mut counter = 10;
    let result = loop {
        counter += 1;
        if counter == 100 {
            break counter;
        }
    };

    println!("Result loop {}", result);

    let mut number = 10;
    while number != 0 {
        number -= 1;
        println!("Reduce number is {}", number);
    }

    let a = [1, 2, 3, 4, 5, 6];

    for item in a.iter().into_iter() {
        println!("A item is {}", item);
    }

    for number in 1..5 {
        println!("number: {}", number);
    }
}

fn my_func(x: i32, y: i32) -> i32 {
    println!("X is {}", x);
    println!("Y is {}", y);

    x + y
}
