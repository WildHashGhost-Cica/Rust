/*Primitive Types :
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 
    Floats: f32, f64
    Boolean (bool)
    Characters (char)
    Tuples
    Arrays
    */
    
pub fn run(){
    let x = 1;

    let y = 2.5;

    let z: i64 = 454545545;

    //Find max size

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greeter: bool  = 10 > 5; 

    //Characters
    let a1 = 'a' ;

    println!("{:?}", (x, y, z, is_active, is_greeter, a1));

    let face = '\u{1F600}';
    println!("{}", face);
    

}