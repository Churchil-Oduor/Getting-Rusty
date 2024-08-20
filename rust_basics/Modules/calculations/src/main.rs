pub mod mathematics;

pub use crate::mathematics::basic::operation;

fn main() {

    let (x, y) = (23.0, 5.0);
    let sum: f32;
    let _abs = operation::_abs(-x);
    let mult = operation::mult(x, y);
    let sub = operation::subtract(x, y);
    let modulus = operation::modulus(x, y);
    let div = operation::divide(x, y);

    sum = operation::add(x, y);
    println!("{x} + {y} = {sum}");
    println!("{x} - {y} = {sub}");
    println!("{x} / {y} = {div}");
    println!("{x} * {y} = {mult}");
    println!("{x} % {y} = {modulus}");
}
