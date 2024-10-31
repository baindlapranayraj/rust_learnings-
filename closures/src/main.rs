#[allow(unused)]
use std::mem::size_of_val;

#[allow(dead_code)]
struct Person {
    first_name: String,
    last_name: String,
}

fn main() {
    bar(); // Understanding Closure syntax

    // crust_of_rust();
}

#[allow(unused)]
fn bar() {
    let x = bar;
    let mut y = 10;

    // Closure with parameter
    let closure = |i| {
        let num = i + 1;
        num
    };

    // Closure with no parameter and it taken y as reference {&y}
    let mut closure_param = || {
        println!("inside Closure with no parameter {}", y);
        y = y + 1;
        y
    };

    println!("The closure call is {}", closure(10));
    println!("Y After call: {}", closure_param());

    let mut number = 10;
    let mut closure_func = |x: i32| {
        let result = x + number;
        number = number + 1;
        result
    };
    println!("The closure function is {}-{}", closure_func(10), number);

    let person_constructor = |first: String, last: String| -> Person {
        Person {
            first_name: first,
            last_name: last,
        }
    };
    let mut perosn_a = person_constructor("Pranay".to_string(), "Raj".to_string());
    let mut change_name = || {
        perosn_a.first_name = "Mani".to_string();
    };
    let f = || {};

    bazz(f);
}

#[allow(unused)]
fn crust_of_rust() {
    let mut a = foo::<u32>;
    println!(
        "The size of foo function in bites is {}",
        std::mem::size_of_val(&a)
    );
    bazz(foo::<u32>);
    bazz(foo::<i32>);
}

fn foo<T>() {}

fn bazz(f: fn()) {
    println!("The size of given function is: {}", size_of_val(&f));
}

// ====================================== Learnings ======================================
// -A closure in Rust is a way to create a function that can capture variables from its surrounding scope.
// -The first is that we did not need to annotate the types of arguments the closure takes or the values it returns.
// -Closures are effectively syntax sugar for traits.
// -We have three sapaerate traits for the Closures
// --Fn (Hear Fn takes &self as reference)
// --FnMut (Hear FnMut takes &mut self as Ref)
// --FnOnce(Hear FnOnce takes self which means it takes entire ownership)
// -The || {} syntax for closures is sugar for these three traits,The || {} syntax for closures is sugar
// for these three traits
