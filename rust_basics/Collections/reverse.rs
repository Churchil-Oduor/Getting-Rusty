fn main() {
    let _str: String = String::from("Hope");

    print_reversed(&_str);

}

fn print_reversed(my_str : &String){
    let mut stack: Vec<char> = Vec::new();
    let mut count: usize;

    count = 0;

    for x in my_str.bytes() {
        stack.push(x as char);
        count += 1;
    }

    for i in 0..(count - 1) {
        println!("{:?}", stack.get(i));
    }
}
