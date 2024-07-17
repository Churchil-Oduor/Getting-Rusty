fn main()
{
    let mut str1: String = String::from("Hello");
    str1.push(',');
    str1.push_str(" world");
    println!("{str1}");
}
