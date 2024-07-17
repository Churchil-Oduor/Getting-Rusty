/**
 * main - entry point.
 */
fn main()
{
    let s: String = "Hello Rusty".to_string(); // converting the string literall into String
                                               // type
    let str1: &str = s.as_str();

    println!("{}", str1);
}
