//Arrays : Fixed list, where elements are the same data types
use std:: mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    //Get single val
    println!("{}", numbers[2]);

    //Re-assign value

    numbers[2] = 20;
    println!("{:?}", numbers);

    //get array length

    println!("Array length: {}", numbers.len());

    //Array are stack allocated
    //We don't need to use std:: if we added it on top 
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

}