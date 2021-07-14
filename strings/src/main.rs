fn print(s: &str) {
    eprintln!("{}", s);
}
fn main() {
    // When to use String and when to use sting slice (&str)
    // * String when you have to modify it
    // * &str when you don't 
    
    // Owned
    let mut my_string: String = String::new(); // Wrapper around Vec<u8>
    my_string.push_str("hello world");
    my_string.push_str("hello world again");
    // Borrow
    let s: &str = "This is a string Literal";
    eprintln!("{}", my_string);
    eprintln!("{}", s.trim());
    print(&my_string);
    print(&my_string);

    let bunny = "1fdasfy".to_string();
    eprintln!("len: {:?}", bunny.len());
}
