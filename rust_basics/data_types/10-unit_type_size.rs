use std::mem::size_of_val;
fn main()
{
    let x: () = ();
    println!("The size of val {}", size_of_val(&x));
    
}
