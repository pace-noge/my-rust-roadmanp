
fn main() {
    // let reference_to_nothing = dangle();
    
    let s = no_dangle();

    println!("{}", s);

}

//fn dangle() -> &String {
//    let s = String::from("hello");

//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
