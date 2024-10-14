fn main() {
    let  num = 10;
    println!("The first value: {num}");
    let num = 11;
    println!("The muted value is: {num}");

    {
        let num = num*10;
        println!("The inner scope value of num is: {num}")
        //-The let num = num*10 will shadow the let num = 11 & the 
        // will ended after the scope ends which means that shadow value works with in 
        // the scope itself

        //--By using let, we can perform a few transformations on a value but have the variable be 
        //  immutable after those transformations have been completed.
    }

    println!("The Outter scope value of num is: {num}")
}


//--Rust force u to think/favour more towards immutability,by default any varaibles in rust 
// is immutable.    