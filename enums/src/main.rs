
#[derive(Debug)]
#[allow(dead_code)]

enum IpAddressKind {
    V4(String),
    V6(String)
}



fn main() {
    let home =  IpAddressKind::V4(String::from("12.0.4.1.6"));
    println!("Home address Enum: {:?}",home);
}
