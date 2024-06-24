fn main()
{
    let _v: () = (); //this is the declaration of a unit type.
                     
    assert_eq!(_v, implicit_unit_type());
    println!("Success!!");
}

/**
 * implicit_unit_type - a function that returns nothing i.e. returns
 * a unit type whose size is zero bytes.
 * */
fn implicit_unit_type()
{
    println!("Hello There!!");
}
