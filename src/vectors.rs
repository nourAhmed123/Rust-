//vectors -resizable arrays
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    //Re-assign value
    numbers[2] = 20;
    // Add on to vector
    numbers.push(6);
    // pop last value
    numbers.pop();
    println!("{:?}", numbers);
    //Get single val
    println!("single value:{}", numbers[0]);

    //Get Vector length
    println!("Vector length: {}", numbers.len());

    //Vectors are stack allocated

    println!("Vector occupies {} bytes", mem::size_of_val(&numbers)); // use std foo2

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice:{:?}", slice); //{:?} debug trait
                                   //Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x)
    }
    //Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers vector : {:?}", numbers)
}
