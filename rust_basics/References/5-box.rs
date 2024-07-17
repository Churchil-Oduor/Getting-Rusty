/**
 * main - entry point.
 */
fn main()
{
    let _x: Box<i32> = Box::new(45);

    let y: Box<i32> = Box::new(-91);

    assert_eq!(*y, -91);
    println!("Success!!");
}
