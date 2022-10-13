fn main(){
    //manipulate strings
    let mut s: String = String::from("Wild");
    println!("{}",s);

    s.push_str("Hash");
    println!("{}",s);
    //another method to manipulate the string
    let mut z: String = String::from("Wild#Ghost");
    println!("{}",z);
    z.replace_range(.., "Ghost");
    println!("{}",z);
}