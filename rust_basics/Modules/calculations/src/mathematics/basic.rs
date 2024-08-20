pub mod operation {
    pub fn add(x: f32, y: f32) -> f32{
        x + y
    }

    pub fn subtract(x: f32, y: f32) -> f32 {
        x - y
    }

    pub fn divide(x: f32, y: f32) -> f32 {
        if y == 0 as f32 {
            println!("Division by Zero Error!!");
            return 0 as f32
        }
        x / y
    }

    pub fn modulus(x: f32, y: f32) -> f32 {
        x % y
    }

    pub fn _abs(x: f32) -> f32{
        if x < 0 as f32 {
            x * (-1 as f32)
        }
        else{
            x
        }
    }

   pub fn mult(x: f32, y: f32) -> f32{
       x * y
   }
}
