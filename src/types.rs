//primitive types ---
//Integers: u8 , i8,u16,i16,u32,i32,u64,i64,u128,i128 (Number of bits they take in memory )
//floats: f32 , f64
//Boolean (bool)
//characters (char)
//Tuples
//Array

// in rust its Not required to set the type for every single variable you create
pub fn run(){
//Default i32
let x =1;

let y=2.5;
//add explicit type 
let z:i64=45546545454545;
//find max size
println!("Max i32: {}",std::i32::MAX);

println!("Max i64: {}",std::i64::MAX);
//Boolean 
let is_active=true;
//Get boolean from expresssion
let is_greater=10>5;
let a1='a';
let face='\u{1F600}';
println!("{:?}",(x,y,z,is_active,is_greater,a1,face))
}