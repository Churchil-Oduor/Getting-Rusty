enum Coin {

    Mia,
    TwoSoo,
    Bob
}

fn value_in_shs(coin: Coin) -> u8 {
    match coin {
        Coin::Mia => 100,
        Coin::TwoSoo => 200,
        Coin::Bob => 1
    }
}

fn main()
{
    let cash = value_in_shs(Coin::Mia);
    println!("{}", cash + value_in_shs(Coin::Bob));
}
