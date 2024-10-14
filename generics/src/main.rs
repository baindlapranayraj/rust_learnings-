#[derive(Debug)]

struct Point<T,U>{
    x:T,
    y:U
}

//The Options and Result are created using generics anf enums
enum Option<K>{
    Some(K),
    None
}

enum Result<I,O>{
    Ok(I),
    Err(O)
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = find_greatest_num(&number_list);
    println!("The largest number is {largest}");

    //++++++++++++ Generics in Structs ++++++++++++++++++
    let mouse_coordinate = Point{x:10,y:7.0};
    println!("{:?}",mouse_coordinate);

    //++++++++++++ Generics in Methods ++++++++++++++++++

    let p1 = Point{x:10,y:20};
    let p2 = Point{x:"Pranay",y:"Rajuuuuu"}; // This would be Point{x:T2,y:U2};
    let p3 = Point{x:"Mani",y:"Rajjjjjj"};
  
   let res = p1.mix_up_points(&p2);
   println!("The value of println would be: {:?}",res);
   println!("The mix_up of p2 and p3 would be: {:?}",p2.mix_up_points(&p3));

}


fn find_greatest_num<T:PartialOrd>(num_list: &[T]) -> &T {
    let mut largest = &num_list[0];

    for num in num_list { 
        if num > largest {
            largest = num;
        }
    }
    largest
}


impl<T,U> Point<T,U> {
    fn mix_up_points<T2,U2:Clone>(self,other:&Point<T2,U2>) -> Point<T,U2> {
        Point {
            x:self.x,
            y:other.y.clone()
        }
    }
}