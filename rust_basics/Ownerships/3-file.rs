fn main()
{
    let s: String = String::from("Hello ");
    let mut s1 = s;

    s1.push_str("world");
    println!("{s1}");
}
