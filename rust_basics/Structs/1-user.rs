/**
 * main - entry point.
 */

fn main()
{
    let mut person1: User = User {
        name: String::from("Basil"),
        age: 32
    };

    person1.name = "Churchil".to_string();
    identifier(&person1);
    person1.name = "Okech".to_string();
    identifier(&person1);
    
}

fn identifier(person: &User){
    println!("New Person is {} aged -> {}", person.name, person.age);
}


struct User{
    name: String,
    age: i32
}
