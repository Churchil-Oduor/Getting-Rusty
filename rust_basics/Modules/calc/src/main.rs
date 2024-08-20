mod mod_add;

use crate::mod_add::addition;

fn main(){

    let (x, y) = (10, 57);

    let sum = addition::add(x, y);
    println!("{} + {} = {} ", x, y, sum);
}
