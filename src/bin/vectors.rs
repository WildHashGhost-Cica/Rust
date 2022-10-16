// Resizeable arrays 

use std:: mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    //Get single val
    println!("{}", numbers[2]);

    //Re-assign value

    numbers[2] = 20;
    println!("{:?}", numbers);

    //get of vector length

    println!("Vector length: {}", numbers.len());

    //Vector are stack allocated
    //We don't need to use std:: if we added it on top 
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

    //add to vector

    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);

    //Pop off last value

    numbers.pop();
    println!("{:?}", numbers);

    //Loop through the vector value

    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    for x in numbers.iter() {
        println!("Number Vec: {:?}", x+100);
    }

    //Loop & mutate values 
    for x in numbers.iter_mut() {
        *x *=2;
    }
    println!("Numbers Vec: {:?}", numbers);
   

}
