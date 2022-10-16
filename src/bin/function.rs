pub fn run(){
    greeting("Hello","WildHashGhost");
    
    println!("{}", add(2,30));

    //Bind function values to variables
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    //Closure 
    let add_nums = |n1: i32, n2: i32| n1 + n2 ;
    println!("Closure Sum: {}", add_nums(3,13));

    let n3: i32 = 100;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3,13));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32{
    n1 + n2
}