use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();

     // Read user input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input into an unsigned integer
    let number: u32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    fizzbuzz(number);
}

fn fizzbuzz(number: u32) {
    if number % 3 == 0 && number % 5 == 0 {
        println!("FizzBuzz")
    } else if number % 3 == 0 {
        println!("Fizz")
    } else if number % 5 == 0 {
        println!("Buzz")
    } else {
        println!("{}", number)
    }
}