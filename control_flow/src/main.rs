fn main() {
    let num = 10;

    if num<5{
        println!("Yes the given number is less then 5");
    }else if num == 5{
        println!("The given number is 5");
    }else{
        println!("The given number is grater then 5");
    };

    let condition = true;

    let number = if condition {5} else {6}; // Like terneray in react.js
    //The values that have the potential to be results from each arm of the if must be the same type
    println!("{number}");
}


//- Blocks of code associated with the conditions in if expressions are sometimes called 🦀 arms.

//-In rust the condition in this code must be a bool. If the condition isn’t a bool, 
// we’ll get an error unlike in js if write a string or any integer it is considered as true 
// but it throws erro in rust
