
struct User {
    name: String,
    age: i32
}
/**
 * main - entry point.
 */

fn main()
{
    let my_name: String = "Churchil".to_string();
    let person1: User = build_user(my_name, 67);
    println!("Name : {}\n Age : {}", person1.name, person1.age);

}

/**
 * build_user - builds a struct user.
 * @name: name of the user.
 * @age: age of user.
 * Return: struct user
 */

fn build_user(name: String, age: i32) -> User {
    User {
        name,
        age
    }
}
