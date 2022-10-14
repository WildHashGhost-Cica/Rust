pub fn run(){
    println!("|Hello from the test.rs file");

    println!("{}",1);
    //bacis Formatting
    println!("{} likes {}", "James", "Dogs");

    //Positional Arguments
    println!("{0} likes {1} and {0} likes {2}", "James", "dogs", "code");

    //Named Arguments
    println!("{name} likes to {activity}", name = "James", activity = "code");

    //Placeholder traits 
    println!("Binary: {:b} Hex:{:x} Octal:{:o}", 10,10,10);

    //Placeholder for debug trait
    println!("{:?}", (12,true, "Hello"));

    //Basic math
    println!(" 10 + 10 = {}", 10 + 10);
}