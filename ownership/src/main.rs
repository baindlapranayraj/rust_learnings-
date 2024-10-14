fn return_string() -> String {
    let name = String::from("Pranay");
    name
}


fn main() {

    //-Primitive data types rust uses copy trait it does not transfer the ownership it rather 
    //  copy the data.

    let mut x = 5;
    let y = x;
    x=10;

    println!("the x:{x} and the y:{y}",);

    let s1 = String::from("hello");
    let _s2 = s1;

    println!("{_s2}, world!");
    // println!("{s1}, world"); This line throw an error since owner ship tranf to _s2 

    //++++++++++++++++++++++++ DeepCopy in Rust ++++++++++++++++++++++++++++++++++++

    let _s3 = _s2.clone();
    println!("The _s2 value is: {_s2} and the _s3 is: {_s3}");
    //-Using .clone() we are explicitly copying heap and the stack.


    let mut m = "Pranay";

    let m1 = &mut m;
    println!("m1:{m1}");
    let m2 = &m;
    println!(" This is m2:{m2}, m1 is");


    // +++++++++++++++++++++ Functional Programs ++++++++++++++++++++++++++++++

    let user_name = return_string();
    println!("The user name is {user_name}");   


    //+++++++++++++++++++++++ Slices in Rust ++++++++++++++++++++++++++++
    
    let slice = String::from("Hello World");
    let slice_1 = &slice[..5];
    let world = &slice[6..11];
    println!("The Given slice_1 is :{slice_1} and the slice_2 is:{world}");

}


//-OwnerShip rules
// -- Each value in Rust has an owner.
// -- There can only be one owner at a time.
// -- When the owner goes out of scope, the value will be dropped.

//-String literals are stored in stack,String literals are immutable unlike String type in Rust 
// which is stored in heap which is mutable,string literals are directly hardcoded,so we know them in
// in compile time and this is string literals are fast and effecient.

//-With the String type, in order to support a mutable, growable piece of text, we need to allocate 
// an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
// --The memory must be requested from the memory allocator at runtime.
// --We need a way of returning this memory to the allocator when we’re done with our String.


//-When a variable goes out of scope, Rust calls a special function for us. This function is 
//called drop, and it’s where the author of String can put the code to return the memory. 
//Rust calls drop automatically at the closing curly bracket.