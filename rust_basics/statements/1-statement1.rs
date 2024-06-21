/**
 * main - entry point of the program.
 * */

fn main()
{
    let (initial_velocity, time, acceleration, result): (f32, f32, f32, f32);
    
    initial_velocity = 0.0;
    time = 30.0;
    acceleration = 10.0;
    result = v_displacement(initial_velocity, time, acceleration);
    println!("The distance travelled is {}", result);
}

/**
 * v_displacement - this function calculates the distance travelled
 * vertically by a projectile in vertical projection.
 *
 * @u: inittial velocity.
 * @t: total time.
 * @a: acceleration
 * Return: the total vertical distance
 * */
fn v_displacement(u: f32, t: f32, a: f32) -> f32
{
    let distance: f32;

    distance = {
        u * t + 0.5 * a * t * t
    };

    return distance;
}
