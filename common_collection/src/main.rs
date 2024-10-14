#[allow(unused_variables)]

fn main() {

  let a: [i32; 4] = [10,12,11,5]; // Gen array stored in stack.

    //- Note that we added a type annotation here. Because we aren’t inserting any values into 
    // this vector, Rust doesn’t know what kind of elements we intend to store.
    
  let mut v: Vec<i32> = Vec::new();
  v.push(10);
  println!("v: {:?}",v[0]);

  let v2 = vec![10,2,8,0,4];
  println!("The v2 value is: {:?}",v2);

  //Reading the Vector

  let read_v: &i32 = &v2[2];
  println!("The value of 2nd index in vector is: {:?}",read_v);
  // - This method directly accesses the element and will panic (crash) if you try to access an 
  // out-of-bounds index, such as v2[10].

  let read_v:Option<&i32> = v2.get(20);

  match read_v {
    Some(read_v)=>{
        println!("The value of 2nd index in using option vector is: {:?}",read_v);
    },
    None =>{
        println!("There is null value on given Index");
    }
  } 

  //-When the get method is passed an index that is outside the vector, it returns None without panicking. 

  let mut vector_1 = vec![1,2,3,45,5];
  vector_1.push(10);
  let first = &vector_1[2];

  println!("The given Vector_1 value is: {:?}",first);

  for i in &mut vector_1 {
     *i=*i+1;
    println!("{i}");
  }

}



// - Vectors size is not known at compile at time becoz it is stored in heap,the size is not 
//   defined hence we can grow or shrink the size whenever we want.

//- When combined with structs, vectors are useful for managing collections of complex data types.



