/**
 * main - entry point of the program.
 * */
fn main()
{
    let b: bool = true;

    let _v = match b { // match is the same as the switch statement in C++
        true => 1,
        false => {
            panic!("We have no value for 'false', but we can panic! ");
        }
    };

    println!("The value of _v is {_v}");

}
