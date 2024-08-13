mod Math {

    pub struct Operations {
        x: i32,
        y: i32,
    }

    impl Operations{
    
        //assocaited function
        pub fn add(x: i32, y: i32) -> i32{
            x + y
        }

        pub fn mult(x: i32, y: i32) -> i32{
            x * y
        }

        pub fn subtract(x: u32, y: u32) -> u32 {
            x - y
        }

        pub fn modulus(x: i32, y: i32) -> i32{
            x % y
        }

        pub fn divide(x: i32, y: i32) -> i32{
            if y == 0 {
                println!("Error Division by 0");
                0
            } else {
                x / y
            }
        }
    }
}
