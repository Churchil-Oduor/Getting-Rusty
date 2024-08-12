mod my {

    #[warn(dead_code)]
    pub struct OpenBox <T> {
        pub contents: T
    }

    pub struct ClosedBox <T> {
        pub contents: T
    }

    impl <T> ClosedBox <T> {
        pub fn new(contents: T) -> ClosedBox <T>{
            ClosedBox {
                contents: contents
            }
        }

        pub fn get_contents(&self) -> &T{
            &self.contents
        }
    }
}

fn main() {
    let _open_box = my::OpenBox { contents: "Hello there" };
    let closed_box = my::ClosedBox::new("Closed Box");


    println!("{}", closed_box.get_contents());
}
