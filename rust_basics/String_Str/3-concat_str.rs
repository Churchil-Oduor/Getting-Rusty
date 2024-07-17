/**
 * main - entry point of the proram.
 */

fn main()
{
    let str1: String = String::from("Hello, ");
    let str2: &str = "world";

    //concatenation
    let str3: String = str1 + str2;
    println!("{}", str3);
}
