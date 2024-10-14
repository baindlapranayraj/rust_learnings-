use std::collections::HashMap;


fn main() {
    let mut score:HashMap<String,i32> = HashMap::new();
    score.insert(String::from("Gojo"),21);
    score.insert(String::from("Geto"),23);
    score.insert(String::from("Yuta"),16);
    score.insert(String::from("Yuji"),19);

    println!("The hash Value is: {:?}",score);

    let a = score.get(&String::from("Gojo"));

    match a {
        Some(value)=>println!("The value is: {}",value),
        None =>println!("The value is none")
    }

    for (key,value) in &score {
        println!("{key}:{value}");
    }

    panic!("This is a panic statement");
   
}


// -Like vectors, hash maps are homogeneous: all of the keys must have the same type, 
// and all of the values must have the same type.


