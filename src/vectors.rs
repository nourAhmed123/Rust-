//vectors -resizable arrays
use std::mem;
pub fn run() {
    //Fixed size vector
    let values: [&str; 2] = ["foo", "bar"];
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut numbers2 = vec![8, 9, 10];
    //Re-assign value
    numbers[2] = 20;
    // Add on to vector
    numbers.push(6);
    // pop last value
    numbers.pop();
    println!("{:?}", numbers);
    //  numbers.clear();
    //  if numbers.is_empty(){
    //     println!("value1 is empty");
    //  }
    //println!("{:?}", numbers);
    //Get single val
    println!("single value:{}", numbers[0]);

    //Get Vector length
    println!("Vector length: {}", numbers.len());

    //Vectors are stack allocated

    println!("Vector occupies {} bytes", mem::size_of_val(&numbers)); // use std foo2
                                                                      //cloning and appending to a vector
    numbers.append(&mut numbers2);
    println!("after append  {:?}", numbers);
    numbers.extend_from_slice(&[4, 5, 6]);
    println!("extend {:?}", numbers);

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice:{:?}", slice); //{:?} debug trait

    let n = numbers.iter().map(|x| x * 2);
    println!("Numbers : {:?}", n);
    //Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x)
    }
    //Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers vector : {:?}", numbers);

    //Testing if a vector contains a value
    if numbers.contains(&3) {
        println!("yes");
    } else {
        println!("No");
    }
}
