// Two type of strings: str and String 

pub fn run(){
    let hello = "Hello World";
    let mut name = String::from("WildhashGhost");

    println!("{} {}", hello, name);

    //Get length
    println!("Length of name:{0} Length of hello: {1}", name.len(),hello.len());

    //Add char 
    name.push('!');
    println!("{}", name);

    //Add String, we can't add to hello if that's in that kind of format

    name.push_str(" Cica");

    println!("{}", name);

    //Capacity in bytes
    println!("Capacity {}", name.capacity());

    //Check if empty 
    println!("Is empty: {}",hello.is_empty());

    //Check Contains
    println!("Contains 'ghost' {}", name.contains("ghost"));

    //Replace
    println!("Repalce: {}",hello.replace("World", "Cica") );

    //Loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    for word in name.split_whitespace(){
        println!("{}", word)
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('C');
    s.push('i');
    s.push('c');
    s.push('a');
    println!("{}",s);

    //Assertion testing, if it's pasted we can't see any messages


    assert_eq!(4, s.len());
    assert_eq!(10, s.capacity());
    
}