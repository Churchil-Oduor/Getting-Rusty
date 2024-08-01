
/**
 * main - entry point.
 */

fn main()
{
    let x = Some(10);
    let sum = plus_one(x);

    let none = plus_one(None);
    println!("{:?}", sum);

}


/**
 * plus_one - takes an option <i32> and if there's a value
 * inside adds 1 to that value. else returns None value.
 *
 * @x: an i32 value or a None value.
 * Return: i32 value + 1 or None value.
 */

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
