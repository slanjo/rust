
use std::string::String;
fn main() {
    let s1 = String::from("Hello");
    println!("s1 = {s1}");
//    let s = "M";
    let s2 = s1.clone();
    
    println!("s1 = {s1}");
    println!("s2 = {s2}");
//    println!("s = {s}");

    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    
    makes_copy(x);
    println!("After function return: {x}");

    let s11 = gives_ownership();
    let s12 = String::from("hello");
    let s13 = takes_and_gives_back(s12);
    println!("s11 = {s11}");
//    println!("s12 = {s12}");
    println!("s13 = {s13}");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
   a_string
}
