fn main() {
    let s1:String = ["first", "second"].concat();
    let s2:String = format!("{}{}", "first_01", "second_02");
    let s3:&str = concat!("first-01", "second-02");

    let s4: String = String::from("test");
    let s5: String = s4 + " okok"; //String type must be first 

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
   // println!("{}", s4); if we want to print out we need to borrow 
   
    println!("{}", s5);
}