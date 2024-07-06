/**
 * main - entry point
 */
fn main()
{
    let str1 = String::from("Hope Brandy");
    let len = _strlen(&str1);
    println!("The length of string -> [{}] is {} characters", str1, len);
}

/**
 * _strlen - functions that calculates the length 
 * of a string.
 * @st: refernce to pointer of string owned by the calling function.
 * Return: len of the string.
 */
fn _strlen(st: &String) -> usize {
    st.len()
}
