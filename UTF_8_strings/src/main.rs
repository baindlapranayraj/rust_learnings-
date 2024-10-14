

fn main() {
// The length u might say it would take 12 hence the 12 bytes but this line takes 24 bytes and len
    
    let hello = String::from("Здравствуйте");
    println!("The length of given string is: {}",hello.len());

    let i = "10";
    let mut str_1 = i.to_string();

    // println!("The value of str_1 is: {str_1} and &str would be: {i}");

    str_1.push_str(" Markes goes to You");
    // println!("The new mutated value of str_1 is: {str_1}");

    let string1 = String::from("Hello ");
    let string2 = String::from("World");

    let result = format!("{string1}{string2}");

    println!("The result for format would be: {result}");

    let emoji = "😁";
    println!("Then len of emoji is: {}",emoji.len());
    println!("{:?}",emoji.bytes());
}

// - A String is a wrapper over a Vec<u8>



