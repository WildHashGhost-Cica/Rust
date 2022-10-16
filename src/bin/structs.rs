//Structs - Used to create custom data types

//traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}


pub fn run(){
   let mut c = Color{
    red: 255,
    blue: 0,
    green: 0,
   };

   println!("Color: {} {} {}", c.red, c.blue, c.green);
}