/**
 * main - entry point of the program
 */

fn main()
{
    let mut s: String = String::from("Hello, ");

    let p: &mut String = &mut s;
    p.push_str("world");
    println!("Success!");
}
