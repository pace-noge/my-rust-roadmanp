
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a string
    s.len()
}  // here, s is out of scope. but beacuse it does not have ownership of what 
   // it reference to, nothing happen
