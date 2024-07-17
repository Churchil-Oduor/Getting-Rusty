/**
 * main - entry point.
 */
fn main()
{
    let mut str1 = String::from("Hello");
    modify(&mut str1);
    println!("{}", str1);
}

/**
 * modify - modifies the string passed onto it.
 * @ptr: mutable borrowed pointer.
 */

fn modify(ptr: &mut String)
{
   ptr.push_str(", World");
}

