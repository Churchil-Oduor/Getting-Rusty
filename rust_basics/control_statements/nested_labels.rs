#![allow(unreachable_code, unused_labels)]

fn main() {
    'outier: loop {
        println!("Entered the outer loop");
        'innee: loop {
            println!("Entered the inner loop");
            break 'outier;

        }
        println!("Unreachable code");
    }
    println!("Exited the outer loop");
}
