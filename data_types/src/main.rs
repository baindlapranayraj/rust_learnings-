use std::io::stdin;

fn main() {
    let _a = 19; //by default integers are types as i32 in rust

    let num: u8  = 254;
    println!("{num}");

    //-signed range is -(2^(n - 1)) to 2^(n - 1) - 1;
    //-unsigned range is 0 to 2^(n - 1);

  println!("+++++++++++++++++ Tupels +++++++++++++++++");

  let tup = ("pranay",19,true);
  println!("{:?}",tup);// Pointing the each tuple using index

  let (name,age,male_bool) = tup; // Destructuring just like in js

  println!("My name is {name}, who is {age} year old like to say I am proud male{male_bool}");

  println!("+++++++++++++++++ Array +++++++++++++++++");

  let arr = [1,2,3,4,5];
  println!("{:?}",arr[1]);

  let arr_second:[i8;5] = [1,2,3,4,5]; // let variable_name:[type;num_of_elements]
  println!("{:?}",arr_second);


  let arr_three = [100,200,300,400,500];
  println!("Enter your index: ");

  let mut input = String::new();

  stdin()
  .read_line(&mut input)
  .expect("Not valid input");

  let input:usize = input.trim().parse().expect("Not valid input");

  println!("The value of the element at index {input} is: {:?}",arr_three[input]);
  
}


//-Each unsigned variant can store numbers from  0 to 2^(n - 1), inclusive,
// where n is the number of bits that variant uses. and for signed is  -(2^(n - 1)) to 2^(n - 1) - 1 inclusive

//+++++++++++++++++ Tupels +++++++++++++++++

// -A tuple is a general way of grouping together a number of values with a variety 
// of types into one compound type. 

//- Tuples have a fixed length: once declared, they cannot grow or shrink in size.

// -Like JS The variable tup binds to the entire tuple because a tuple is 
// considered a single compound element.

// +++++++++++++++++ Array +++++++++++++++++

//-Arrays are useful when you want your data allocated on the stack rather than the heap if 
// we know the fixed size at compile time

// -Vectors in Rust are dynamic arrays whose size can change at runtime. Because vectors can grow or shrink, 
// their data is stored on the heap.
