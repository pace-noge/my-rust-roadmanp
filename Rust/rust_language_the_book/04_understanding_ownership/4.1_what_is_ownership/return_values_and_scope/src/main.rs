
fn main() {
    let s1 = gives_ownership();
    
    let s2 = String::from ("hello");

    let s3 = takes_and_gives_back(s2);

    let s3 = String::from("hello");         // new variable s3

    let (s4, len) = calculate_length(s3);    // s3 passed to calculate_length (s3 moved to method, it's not available anymore in this scope
    println!("The length of '{}' is {}.", s4, len);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();

    (some_string, length)
}


