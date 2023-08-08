// Few basic usecases in Rust
// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

use rand::Rng;

fn main() {
    println!("Hello, world!");
    my_function(42);

    // Multpiply 3 * 42
    let mut result = multiply(3, 42);
    println!("3 * 42 = {}", result);

    if result < 0 {
        println!("Negative");
    }
    else if result > 0 {
        println!("Positive");
    }
    else {
        println!("Null");
    }

    result = 4;
    println!("result = {result}");

    //let mut random_number: u32;
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        if secret_number % 5 == 0 {
            println!("{secret_number} IS divisible by 5");
            break;
        }
        else {
            println!("{secret_number} IS NOT divisible by 5");
        }
    }
    
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 3;
        }
    };
    println!("result = {result}");

    // loop in an array
    let a = [1, 1, 2, 3, 5, 8];
    let mut index = 0;

    // while style
    while index < a.len() {
        println!("Value at index {index} is {}", a[index]);
        index += 1;
    }

    // for each element style
    index = 0;
    for element in a {
        println!("Value at index {index} is {element}");
        index += 1;
    }
}

fn my_function(x: i32) {
    println!("Called function 'my_function' with argument {}", x);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}