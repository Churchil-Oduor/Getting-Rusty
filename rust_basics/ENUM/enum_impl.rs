enum Packet {
    hostIP(String),
    destinaionIP(String)
}

impl Packet {
    fn ping(&self) {
        println!("pinging to ip");
    }
}

fn main()
{
    let hostIp = Packet::hostIP("127.0.0.1".to_string());
    hostIp.ping();

}
