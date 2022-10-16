struct Color(u8, u8, u8);

pub fn run(){
    let mut c =  Color(255, 0, 0);

    println!("Color: {} {} {}", c.0, c.1, c.2);

    c.0 = 200;
    println!("Color: {} {} {}", c.0, c.1, c.2);
}