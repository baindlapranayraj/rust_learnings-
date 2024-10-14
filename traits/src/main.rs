#[allow(unused_variables)]

#[allow(dead_code)]
trait Summery {
    fn summerize(&self)->String; // Function Decleration 

    fn summerize_authour(&self)->String{
       let content = format!("Read more about Authour......");
       content
    }
}

struct NewsArtical {
    headline:String,
    location:String,
    author:String,
    content:String
}

#[allow(dead_code)]
struct Tweet {
    user_name:String,
    content:String,
    reply:bool,
    retweet:bool
}

impl Summery for NewsArtical {
    fn summerize(&self)->String {
       let content = format!(
            "The auther for headline: {} is {} and location is: {} and content is:{}",
            self.headline,self.author,self.location,self.content
        ); 
        content
    }
}

impl Summery for Tweet {
    fn summerize(&self) -> String {
        let content = format!(
            "The user: {} tweetd this {} give your thoughts",
            self.user_name,self.content
        );
        content
    }
}

fn main() {
     let news_artcile = NewsArtical{
        headline:String::from("Rust is Fu**ing Awsome"),
        location:String::from("Hyd,DullapallyXRoad"),
        author:String::from("Pranay Raj"),
        content:String::from("Go and learn Rust Dude")
     };

     let tweet = Tweet {
        user_name:String::from("pranay.dev"),
        content:String::from("Some time Rust reminds me typescript they both have some similar characters"),
        reply:true,
        retweet:false
     };

    info_aggregator(&tweet,&news_artcile);
}


fn info_aggregator(source1: &impl Summery, source2: &impl Summery){
    println!("{}",source1.summerize());
    println!("{}",source2.summerize());
}


