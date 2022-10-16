// While Loop

pub fn run(){
    let mut count = 0;
    while count <= 100 {
        if count % 15 == 0{
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
    
    // need to increment INC
    count += 1;
    }
    
    

}