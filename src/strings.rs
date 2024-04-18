//Primitive str= Immutable fixed length string somewhere in memory
//string=Growable,heap-allocaated data structure - use when you need to modify or own string data

pub fn run(){
let mut hello= String::from("Hello ");
//Get length
println!("Length:{}",hello.len());
hello.push('w'); //push for one charachter
hello.push_str("orld!"); //push string 

//Get capacity no of bytes can store
println!("capacity : {}",hello.capacity());
//check if empty
println!("Is Empty: {}",hello.is_empty());
println!("{}",hello);
//contains
println!("contains 'world'{}",hello.contains("world"));
//replace
println!("Replace: {}",hello.replace("world", "there"));
//Loop through string by whitespace
for word in hello.split_whitespace(){
    println!("{}",word);
}
//create string with capacity
let mut s=String::with_capacity(10);
s.push('a');
s.push('b');
//Assertion testing 
assert_eq!(2,s.len()); //lw 3 hatdena error 
println!("{}",s);
}