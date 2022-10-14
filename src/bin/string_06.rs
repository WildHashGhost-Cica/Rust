fn main(){
    //index 
    let m:String = "Wildhashghost".to_string();
    let m1:String = m[0..8].to_string();

    println!("{}", m);
    println!("{}", m1);

    let s1: &str = "😀 😀 😀 😀 😀";
    let s2: &str = &s1[0..3];

    println!("{}", s2);
}