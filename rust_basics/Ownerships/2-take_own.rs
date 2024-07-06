/**
 * main - entry point of the program
 * */
fn main()
{
    let s1: String = String::from("Rustacean");
    let s2: String = take_ownership(s1); // ownership of the text -
                                         // Rustacean has been transferred to take_ownership.
                                         // s1 is no longer valid here.

    println!("{s2}");

}

/**
 * take_ownership - takes the ownership from s1.
 * 
 * @s: argument handing the ownership to take_ownership function.
 * Return: modified version of s
 * */
fn take_ownership(s: String) -> String {

    let res: String = String::from(format!("Hello {s}"));
    return res;
}
