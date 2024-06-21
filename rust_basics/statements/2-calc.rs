/**
 * main - entry point of the program.
 */
fn main()
{
    let sum: i32 = sum(2, 10);
    let difference = difference(10, 90);
    let product = product(10, 40);
    let sqr = sqr(3, 3);

    assert_eq!(sum, 12);
    println!("Sum of 2 and 10 is {}", sum);
    assert_eq!(difference, -80);
    println!("Difference of 10 and 90 is {}", difference);
    assert_eq!(product, 400);
    println!("Product of 10 and 40 is {}", product);
    assert_eq!(sqr, 27);
    println!("Square of 3 and 3 is {}", sqr);

    println!("Success!!");
}

/**
 * sum - sums two numbers x and y of type i32.
 * @x: first i32 integer.
 * @y: second i32 integer.
 * Return: sum of x and y.
 */
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

/**
 * difference - subtracts x and y.
 * @x: input to subtract y from.
 * @y: input to subract x from.
 * @Return: difference between x and y.
 * */
fn difference(x: i32, y: i32) -> i32{
    x - y
}

/**
 * product - calculates the product of x and y.
 * @x: input integer to be multiplied.
 * @y: input integer to be multiplied.
 * @Return: product of x and y.
 * */
fn product(x: i32, y: i32) -> i32{
    x * y
}

/**
 * sqr - calculates the square of x raised to y.
 * @x: integer to be raised.
 * @y: integer to raise x by.
 * Return: x raised to power y.
 * */
fn sqr(x: i32, y: i32) -> i32 {

    let mut ans: i32 = 1;

    for _i in 1..=y
    {
        ans *= x;
    }
    ans
}









