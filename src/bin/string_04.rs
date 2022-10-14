fn main(){
    let s1: String = String::from("tic");
    let s2: String = String::from("tac");
    let s3: String = String::from("toe");
    
    let s: String = format!("{} {} {}", s1, s2, s3);

    println!("{}",s1);
    println!("{}",s2);
    println!("{}",s3);
    println!("{}",s);

    // using string slice 
    let o: String = format!("{}-{}-{}", s1, "tac", s3);

    println!("{}", o);
}