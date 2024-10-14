#[derive(Debug)]
#[allow(unreachable_patterns)]

#[allow(dead_code)]
enum UsState {
    Alaska,
    Alabama,
    Washington,
    NewYork
}
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

    fn value_in_cents(coin:&Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) =>{
                println!("The {:?} Qurter to Cents is 25",state);
                25
            }
        }
    }


fn main() {
  value_in_cents(&Coin::Quarter(UsState::Washington));
}





// ++++++ if VS matches ++++++++
// - if is simple it checks bool and have bascic conditioning branches
// - No exhastive checking in if and there is no pattern matching 

// - match is sophesticated and have advance pattern matching.
// - Ensure all cases are handled and it is concise and expressive.
// - Supports value binding and allows direct return value.



