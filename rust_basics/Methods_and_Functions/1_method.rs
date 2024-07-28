struct Circle {
    radius: f32
}

impl Circle {
    fn area(&self) -> f32 {
        3.142 * self.radius * self.radius
    }

    fn circumference(&self) -> f32 {
        2.0 * 3.142 * self.radius
    }
}

fn main ()
{
    let circle: Circle = Circle{radius: 7.0};

    let area: f32 = circle.area();
    let perimeter: f32 = circle.circumference();

    println!("{:?}", perimeter);
    
}
