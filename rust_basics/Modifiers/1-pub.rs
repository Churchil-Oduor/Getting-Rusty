mod my_mod {
    fn private_function() {
        println!("called my_mod::private_function()");
    }

    pub fn function() {
        println!("called my_mod::function()");
    }

    pub fn indirect_access() {
        private_function();
    }

    pub mod nested_mod {
        pub fn function() {
            println!("called my_mod::nested_mod::function");
        }

    }
}


fn main() {
    my_mod::nested_mod::function();
}
