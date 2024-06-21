use std::io;

fn main() {
    println!("Welcome to our guessing game!");


    println!("Enter a number! ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input value!!");

    println!("You entered the number {guess}");

}
