use std::fs::File;

fn main() {
    let res:Result<i32,String> = divide(10,1);
    println!("The result is: {:?}",res);

    match res {
        Ok(value)=>println!("The result is: {value}"),
        Err(_)=>println!("The give value is not appropiate")
    }

    let file = File::open("hello.txt");

    let response = match file {
        Ok(file)=>file,
        Err(err)=>match err.kind(){
            std::io::ErrorKind::NotFound =>match File::create("hello.txt"){
                Ok(file)=>file,
                Err(e)=>panic!("Error while creating file:{:?}",e),
            },
            _=>panic!("Something is wrong")
        },
    };

}

fn divide(x:i32,y:i32) -> Result<i32,String> {
    if y == 0 {
        Err("The given Value is not appropiate".to_string())
    }else{
        Ok(x/y)
    }
}