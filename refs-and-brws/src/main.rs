fn main() {
    let s1 = String::from("hell on earth");

    let len_ref = calculate_length_ref(&s1);
    println!("The length of '{s1}` is {len_ref}");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    //this function takes the ownership of s1 once called

    let length = s.len(); // len() returns the length of a String

    (s, length)

}

fn calculate_length_ref(s: &String) -> usize {

    s.len()

}
