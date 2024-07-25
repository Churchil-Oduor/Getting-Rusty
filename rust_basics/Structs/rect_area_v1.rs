
/**
 * main - entry point.
 */
fn main()
{
    let rect_1 = (68, 34);
    let area = rect_area(rect_1);
    println!("The area of rectangle: L -> 68 and width -> 34 is {}", area);
}

/**
 * rect_area -  calculates the area of rectangle.
 * @dimensions: tuple struct contaning the dimensions of the rectngle.
 *
 * Return: Area of the rectangle.
 */

fn rect_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
