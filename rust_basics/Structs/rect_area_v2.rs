
/**
 * Rectangle - structure of the rectangle.
 */

struct Rectangle {
    width: u32, 
    height: u32
}

/**
 * main - entry point.
 */
fn main()
{
    let rect_1 = Rectangle {
        height: 45,
        width: 57
    };

    
    let area = rect_area(&rect_1);
    println!("The area of rectangle: L -> {} and width -> {} is {}", rect_1.height, rect_1.width, area);
}

/**
 * rect_area -  calculates the area of rectangle.
 * @dimensions: tuple struct contaning the dimensions of the rectngle.
 *
 * Return: Area of the rectangle.
 */

fn rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
