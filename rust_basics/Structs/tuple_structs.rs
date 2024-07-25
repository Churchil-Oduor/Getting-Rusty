fn main()
{
    let name: String = "Churchil".to_string();
    let point = point_1(2, 3, name);
    println!("{}", point.2);
}

struct point_1(i32, i64, String);
