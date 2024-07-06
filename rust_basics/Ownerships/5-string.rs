/**
 * main - entry point
 * */
fn main()
{
    let t: (String, String) = (String::from("Chacho"), String::from("Basil"));
    let _s: String = t.0; //ownership of string at index 0
                          //is transferred from tuple t to string _s
                          //hence t.0 is invalid from here hence forth.
                          //
    println!("{}", t.1); 
}
