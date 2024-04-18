use std::result;


fn say_hello_world()  //->String  specify we are returning a string 
{
let message =String::from("Hello,world!");
println!("{}",message);
}
// Format! Useful when you need the formatted string for something other than immediate display, such as storing it, sending it over a network, or including it in a larger data structure.

fn hello(to_person: String)->String{
    format!("Hello,{}!",to_person)
}
fn greeting(greet: &str,name: &str){
println!("{} {} nice to meet you ",greet,name)
}
fn add(n1: i32, n2:i32)-> i32{
    n1+n2
}
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
fn process_name(name: &str, callback: fn(&str)->()){
    callback(name);
}
pub fn run(){
    process_name("Nour", greet);
say_hello_world();
let hi=hello(String::from("John"));
println!("{}",hi);
greeting("Hello", "Nour");
//bind function values to variable
let get_sum=add(5, 6);
println!("sum {}" , get_sum);

//inline functions 
let say_hello_to=|name: &str|format!("hello {}",name);
println!("{}",say_hello_to("NOURR"));

// closure
let n3:i32=10;
let add_nums=|n1:i32,n2:i32|n1+n2+n3;
println!("closure sum: {}",add_nums(3,3));

let mult_by_2=|n1:i32|n1*2;
let ptr=mult_by_2;
let result =ptr(10);
println!("{}",result);
println!("Result {}",mult_by_2(20));
//Multiple statments in inline functions needs curly brackets
let ask_for_age=||{
 //Ask the user for age
 //calculate how old in 10 years
 
};
}

