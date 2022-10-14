fn main(){
    //concatenating two strings 
    //Operator

    
        let s1: String = String::from("Hello, ");
        let s2: String = String::from("world");
        let s3: String = s1 + &s2;

        
        println!("{}", s2);
        println!("{}", s3);
    
}