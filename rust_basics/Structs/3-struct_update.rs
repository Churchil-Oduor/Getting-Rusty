/**
 * main - entry point.
 */

fn main()
{
    let mut person1: User = User {
        name: String::from("Basil"),
        age: 32,
        city: "ENFJCHKDO".to_string(),
        email: "churchil@gmail.com".to_string()
    };

    person1.name = "Churchil".to_string();
    identifier(&person1);
    person1.name = "Okech".to_string();
    identifier(&person1);

    //testing struct update syntax here
    let person2: User = User {
        name: "Peter".to_string(),
        email: "peter@gmail.com".to_string(),
        ..person1
    };
    identifier(&person2);
    
}

fn identifier(person: &User){
    println!("New Person is {} aged -> {}\n Email -> {}, City -> {} ", person.name,
             person.age, person.email, person.city);
}


struct User{
    name: String,
    age: i32,
    email: String,
    city: String,
}
