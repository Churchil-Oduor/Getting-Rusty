/**
 * main - entry point.
 */

fn main()
{
    let mut s: String = String::from("hello");
    concat(&mut s);
    println!("{}", s);
}

/**
 * concat - concatenates strings.
 * @s: pointer to string from lending function.
 */

fn concat(s: &mut String)
{
    s.push_str(" world");
}
