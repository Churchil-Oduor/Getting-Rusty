/**
 * main - entry point of the program.
 */

fn main()
{
    struct Car
    {
        car_type: String,
        brand: String,
        plate_number: Box<u8>,
    }

    let my_car: Car = Car {
        car_type: String::from("Bently"),
        brand: String::from("Vish"),
        plate_number: Box::new(2),
    };

    //destructuring my Car and assigning the values to other 
    //variables which become the owner.
    let Car { car_type, plate_number, ref brand} = my_car;
    println!("My car type is {}\nplate number {}\nbrand {}", car_type, plate_number, my_car.brand);



}
