use std::io;

fn main() {

    println!("Welcome to the guessing game!! \n\n");
    println!("Enter a number : ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    println!("You guessed the number {guess}");

}
