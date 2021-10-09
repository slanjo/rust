use std::io;
fn main() {
    // "!" - signifies a macro prntln! is a macro because it can take a different 
    // number of arguments
    println!("Enter your weight (kg): ");
    let mut input = String::new(); //Strings live on the heap

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    dbg!(weight);
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
 //the last expression that doesn't have semi-colon  means "return"
    (weight / 9.81) * 3.711
}

//Each value in Rust is owned by a variable. 

//When the owner goes out of scope, the value will be dealocated.

//There can be only ONE owner at a time. 
