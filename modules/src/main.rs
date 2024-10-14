mod garden;
mod dishes;
use crate::dishes::salad::summer_farro_salad;

fn main() {
    println!("Is tomato is planted: {}",garden::vegetables::tomato("🍅"));
    println!("IS summer_farro_salad is done: {}",summer_farro_salad());
}
