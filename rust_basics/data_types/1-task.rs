/**
 * main - entry point of the application
 *
 * */

fn main()
{
    let y: u32 = 100;
    let casted_y: u16 = cast(y);

    println!("Successfully casted y -> {casted_y} to u16 from u32");
}

/**
 * cast - function that casts input from u32 to u16
 * @x: u32 type argument.
 * Return: returns x casted in u16 type.
 * */
fn cast(x: u32) -> u16 {
    return x as u16;
}
