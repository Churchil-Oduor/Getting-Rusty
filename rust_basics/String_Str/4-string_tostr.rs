/**
 * main - entry point.
 */

fn main()
{
    let str1: String = String::from("Jomo ");
    let str2: String = String::from(" Kenyatta");

    let str3: String = str1 + &str2; //conmverted str2 into a string literall
    println!("{}", str3);
}
