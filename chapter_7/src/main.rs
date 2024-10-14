mod garden; // U cannot create nested imports in Rust like mod garden::vegetables
mod fruites;




fn main() {
    let tree = String::from("🌼");
    let res:String = garden::vegetables::plant_tree(&tree);
    println!("Is Trees is Planted:{}",res);
    fruites::pinapple::pinco();
}



//-impl (used to define methods or implement traits) and mod (used to declare modules) are not the 
// same in Rust.