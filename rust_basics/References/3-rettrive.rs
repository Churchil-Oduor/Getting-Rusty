/**
 * main - entry point.
 */

fn main()
{
    let x: i32 = 5667;
    let ptr: &i32 = &x;

    assert_eq!(x, *ptr);
    println!("Success!!");
}
