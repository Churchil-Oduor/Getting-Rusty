/**
 *
 * main - entry point.
 */

fn main()
{
    let name: &str = "Churchil";

    //pass name as str and not &str
    greetings(String::from(name));

}

/**
 * greetings - sends out greetings to passed name.
 * @name: person to be greeted.
 */
fn greetings(name: String)
{
    println!("Hello {}", name);
}
