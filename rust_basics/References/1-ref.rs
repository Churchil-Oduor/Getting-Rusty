/**
 * main - entry point.
 */

fn main()
{
    let x: i32 = 39;
    let ptr: &i32 = &x;

    println!("Memory address of p is {:p}", ptr);
}
