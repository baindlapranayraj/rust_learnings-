use std::io::stdin;


#[allow(dead_code)]
#[derive(Debug)]

struct Diameter {
    width:u64,
    height:u64
}

impl Diameter {
 fn calculate_area(&self) -> u64 {
        let width:u64 = self.width;
        let hight:u64 = self.height;
        let area:u64 = width*hight;
        area
    }
    fn width_bool(&self) -> bool{
        if self.width > 10{
            true
        }else{
            false
        }
    }
}


fn main() {

    let mut width = String::new();
    let mut height = String::new();
    println!("To calculate the area of rectangle");

    println!("The Enter the Width: ");
    stdin().read_line(&mut width).expect("Not a valid number");

    println!("The Enter the Height: ");
    stdin().read_line(&mut height).expect("Not a valid number");


    let variables:Diameter = Diameter {
        width:width.trim().parse().expect("Not a valid Number"),
        height:height.trim().parse().expect("Not a valid Number")
    };

    // println!("The Reactangle Parameter Struct :{:?}",variables);

    let reactangle_area = variables.calculate_area();
    println!("The area of given rectangle is: {reactangle_area}");
    println!("The given width is greater the 10m which is: {}",variables.width_bool());

    // +++++++++++++++++++++++++ Associated Functions ++++++++++++++++++++++++++++

    
}
