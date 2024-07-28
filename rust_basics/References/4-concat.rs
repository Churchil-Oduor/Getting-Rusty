/**
 * main - entry point.
 */

fn main()
{
    let mut s: String = String::from("hello");
    concat_world(&mut s);
    println!("{}", s);
}

/**
 * concat_world - concatenates strings.
 * @s: pointer to string from lending function.
 */

fn concat_world(s: &mut String)
{
    s.push_str(" world");
}


