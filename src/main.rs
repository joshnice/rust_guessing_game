// imports
// std - standard library
// io - input and output
use std::io;
// rand - create that is a dependency
// Rng - trait that random number generators implement
use rand::Rng;
// std - standard library
// cmp - comparsion? 
// ordering import
use std::cmp::Ordering;

// entry point for the application
fn main() {
    // logging
    println!("Welcome to the guessing game!");

    // rand is the create we are using
    // thread_rng is a function we want to use
    // gen_range is method we want to use
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {

        println!("Please input your guess");

        // mut makes it mutable 
        // it is also a string
        // String::new() - new is an associated function to String 
        let mut guess = String::new();

        // using the import library io
        io::stdin()
            // read line method, needs a mutable string so it can be overwritten
            // & means it is a reference, allows the code to access the same data mutiple times without needing to copy it
            .read_line(&mut guess)
            .expect("Failed to read line");

        // uses shadowing to allow a var called the same thing be reassigned to a different type 
        // create a var caled guess of type unsigned 32-bit integer
        // trim removes any spaces
        // parse changes it to a number typed, number type is defined by the variable it is assigned to
        // expect is called if the parsing fails
        let guess: u32 = match guess.trim().parse() {
            // once parsed check is num if so return num
            Ok(num) => num, // has to match the pattern to return (condition)
            // if error then continue
            // underscore is a catch all value
            Err(_) => continue, // continue goes to the next iteration of the loop
        };

        // {} is a placeholder you can have a number of them, the order of the placeholders match the order of the vars in the print command
        println!("You have guessed: {}", guess);

        // takes the guess value and compares it to another value
        // in this case guess is compared to secret number
        // returns an ordering enum of equal, greater or less, depending on the enum returned, chooses what bit of code is ran
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win");
                break; // breaks out of the loop
            }, 
            Ordering::Greater => println!("Too big"), // these are called arms
            Ordering::Less => println!("Too Small"),
        }
    }

}
