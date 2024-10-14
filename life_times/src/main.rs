//- <'a> generics tells that string_1 and string_2 has same life time,Since the logest_string fucn dont know
fn logest_string<'a>(string_1: &'a String, string_2: &'a String) -> &'a String {
    if string_1.len() > string_2.len() { string_1 } else { string_2 }
}

fn bool_func()->bool{
    false
}
fn main() {
    //+++++++++++++++++++++++ Generics Lifetimes in functions +++++++++++++++++++++++
    let string_1 = String::from("Pranay");
    let mut res: &String;
    let foo = String::from("Mani Raj");
   
     let string_2 = String::from("Raj");
     res = logest_string(&string_1, &string_2);
     println!("{res}");
   
    if bool_func(){
        res = &foo;
    }
    println!("{res}");

    // +++++++++++++++++++++ More Examples for Lifetimes +++++++++++++++++++++++++++++++

    let mut vector = vec![1,2,3,4,5];
    let ref_vec = &vector[1];
    //  vector.push(10);
    println!("{:?}-{:?}",ref_vec,vector);
}

// - Lifetimes are named regions of code that a reference must be valid for.

// -Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that
//  it refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter
//  than 'a: the subject of the reference doesn’t live as long as the reference.
