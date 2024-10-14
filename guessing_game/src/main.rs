use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the Number");
    let secret_number: u8 = rand::thread_rng().gen_range(1..=10);

    loop {
    println!("Please input your guess.");
    let mut guess = String::new();
    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess:u8 = match guess.trim().parse() {
        Ok(num)=>num,
        Err(_)=>continue,
    };

    println!("Your guesse is: {guess}");

    match guess.cmp(&secret_number) {
       Ordering::Less => println!("Your Guess is too small dude"),
       Ordering::Greater => println!("Wow thats a high number guess"),
       Ordering::Equal => {
        println!("Congrates you have guessed it right!!!!");
        break;
      }
     }
    }
        
}







//- In Rust by default variables are immutable meaning once u assign a variable to a value, the value
// Won't change

//- If u want to make them mutable we add mut keyword before variable like in line no: 8

//- & creates a reference (points to a value without taking ownership). similar to how in js 
//  store its objects in heap and its object by refernce,Which mean using we only borrow the data 
//  from the respective owner so we use the data but we cannot change/mutate the data inorder to 
//  mutate the data we use mut keyword: &mut


//- * dereferences a reference (gives the value the reference points to).


