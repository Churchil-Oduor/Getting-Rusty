struct Square {
    width: u32,
    length: u32
}

impl Square {
    fn square(len: u32) -> Self {
        Self {
            width: len,
            length: len
        }
    }
    
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn main()
{
    let square_1: Square = Square::square(7);
    println!("Area {}", square_1.area());
}
