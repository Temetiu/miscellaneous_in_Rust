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
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 3;
        }
    };
    println!("result = {result}");

}

fn my_function(x: i32) {
    println!("Called function 'my_function' with argument {}", x);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}