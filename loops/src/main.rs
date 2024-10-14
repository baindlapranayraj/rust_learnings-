fn main() {

  let mut count = 0;
  let result = loop{
    count = count+1;
    if count==10{
        break count*10;
    }
   };

   println!("{:?}",result);

 //++++++Looping an array+++++
   let arr = [1,2,3,4,5];

   let mut index = 0;
   while index < arr.len() {
    println!("The given index {index} value is {:?}",arr[index]);
    index += 1;
   };

   for element in arr {
    println!("The value is: {element}");
   };

   //The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
}

//-Rust has three kinds of loops: "loop", "while", and "for"