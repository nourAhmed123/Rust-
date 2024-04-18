//Tuples group together values of different types
//Max 12 elements
//Tuples doesnot have field names 
//Tuple struct
//accessing tuples with dot notation 
struct Point (f64,f64,f64);
pub fn run(){
let point=Point(0.0,1.0,2.0);
let person:(&str,&str,i8)=("Brad","Mass",37);
println!("{} is from {} and is {}",person.0,person.1,person.2);

}