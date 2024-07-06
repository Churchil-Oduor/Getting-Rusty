fn main()
{
    let x: Box<i32> = Box::new(23); //box returns a pointer 
                                    //in the heap
    let mut y: Box<i32> = Box::new(12);

    *y = 56; //derefencing the pointer -> y to assign
             //the value 56 to the location.
    assert_eq!(*x, 23);
    println!("Success!!");
}
