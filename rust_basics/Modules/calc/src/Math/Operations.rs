mod operations {


    pub mod advanced {
        pub fn power(num: u32, pow: u32) -> u32{
            let temp: u32 = num;

            for i in 0..pow{
                num *= temp;
            }

            num
        }
    }

    pub mod simple{
        pub fn add(x: u32, y: u32) -> u32 {
            x + y
        }

        pub fn subtract(x: u32, y: u32) -> u32{
            x - y
        }

        pub fn divide(x: u32, y: u32) -> u32{
            if y == 0 {
                println!("Division by zero Error");
            }
        }

        pub multiply(x: u32, y: u32) -> u32{
            x * y
        }
    }
}
