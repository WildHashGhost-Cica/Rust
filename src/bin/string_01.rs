fn main(){
    //create strings
    let s1: &str = "Hello world ";
    println!("{}", s1);

    let s2: String = String::from("Hello Wild");
    println!("{}", s2);

    let s3:String = "Hello WildHash".to_string();
    println!("{}", s3);

    let s4: String = "Hello WildHashGhost".to_owned();
    println!("{}", s4);
    //borrowed string from s4 [..] is mean the entire string
    let s5:&str = &s4 [..];
    println!("{}", s5);
}
    
