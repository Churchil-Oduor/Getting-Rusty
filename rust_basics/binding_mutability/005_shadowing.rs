fn main()
{
    let x: i32 = 10;

    {
        let x = 34;
        assert_eq!(x, 34);

    }

    assert_eq!(x, 10);
    let x: i32 = 23;
    println!("{}", x);
}
