fn main()
{
    let (mut x, _y) = (12, -3);
    x += 2;
    assert_eq!(14, x);
    println!("Success!!");
}
