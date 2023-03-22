#[allow(dead_code)]
#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    // etc...
    RhodeIsland,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState)
}

impl Coin {
    fn value(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn main() {
    println!("A Coin::Penny is worth {} cents!", Coin::Penny.value());
    println!("A Coin::Quarter is worth {} cents!", Coin::Quarter(USState::RhodeIsland).value());

    let coin = Coin::Quarter(USState::Alabama);
    let res = if let Coin::Quarter(_state) = &coin {
        coin.value()
    } else {
        0
    };

    println!("The result is {}!", res);
}
