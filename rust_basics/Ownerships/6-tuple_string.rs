/**
 * main - entry point.
 */

fn main()
{
    let t: (String, String) = (String::from("Hello"), String::from("World"));

    //cloning so that t retains ownership.
    let (s1, s2) = t.clone();
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}
