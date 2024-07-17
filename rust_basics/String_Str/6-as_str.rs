fn main()
{
    let str1: String = String::from("Churchil");
    let str2: String = String::from(" Basil");

    let str3: String = str1 + str2.as_str();

    println!("{}", str3);
}
