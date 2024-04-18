#![deny(clippy::all)]
//variables hold primitive data or refrences to data
//variables are immutable by default
//rust is a block scoped langugage
//Mutability is the ability to change the data the data inside a variable with the same dataype
//i32 i signed values cam comtain + or -
//u32 unsigned only positive 
//shadowing same variable with the different data types
const MY_AGE: u8 =22;
pub fn run(){
let name= "Brad";
let mut age=37;
println!("MY name is {} and i am {}",name,age);

age=38;
println!("MY name is {} and i am {}",name,age);
//Define constant
const ID: i32=001;
println!("ID : {}",ID);
//Assign multiple vars
let (my_name,my_age)=("Brad",37);
println!("{} is {}",my_name,my_age);
//shadowing
let data ="FOO";
{
    let data =data.to_string();
}

println!("MY AGE IS {}",MY_AGE);
}