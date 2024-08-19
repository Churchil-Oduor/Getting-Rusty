struct Point {
    x: f64,
    y: f64
}

impl Point {
    //this is an associated function
    //Can be called without instance
    fn origin() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    fn new(x: f64, y: f64) -> Point{
        Point {x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point
}

impl Rectangle {
     //this is a method because it must be called by an instance.

    fn area(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }


    fn perimeter(&self) -> f64 {
        let Point {x: x1, y:y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        2.0 * ((x1- x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x:f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;

    }
}


struct Pair(Box <i32>, Box <i32>);


impl Pair {

    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair {}, {}", first, second);

        //first and second get out of scope and are freed
    }
}

fn main() {

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(4.1, 1.3),
    };


    println!("Rectangle perimeter: {}", rectangle.perimeter());
}
