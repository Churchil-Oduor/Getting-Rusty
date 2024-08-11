fn main() {
    let tray = [3, 4, 6];

    match tray {
        [3, x, y] => {
            println!("The first item is 3, second is {} and third is {}",x, y);

        }
        [2, d, g] => {
            println!("{} {}", d, g);
        }
    }
}
