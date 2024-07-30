#[derive(Debug)]
enum IpAddr {
    v4(String),
    v6(String),
}

fn main(){

    let home = IpAddr::v4(String::from("127.0.0.1"));
    println!("{home:?}");


}
