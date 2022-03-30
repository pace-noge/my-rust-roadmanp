
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Coin,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    // match config_max {
    //    Some(max) => println!("The maximum is configured to be {}", max),
    //    _ => (),
   // }
    
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 3;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}
