use my_module::nested::function as other_func;

mod my_module {

    pub mod nested {

        pub fn function() {
            println!("I am a nested function!!");
        }
    }
}


fn main(){

    other_func();
    my_module::nested::function();
}
