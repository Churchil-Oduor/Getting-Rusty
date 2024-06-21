use std::mem::size_of_val;
fn main()
{
    let c1: char = 'a';
    println!("{}", size_of_val(&c1));
}
