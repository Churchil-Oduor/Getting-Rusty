#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

#[derive(Debug)]
struct IPAddr {
    kind: IpAddrKind,
    address: String,
}

fn main()
{
    let home = IPAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    println!("{:?}", home);
}
