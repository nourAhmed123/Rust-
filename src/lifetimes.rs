//sample function that not compile string slice cannot be returned as the string is borrowing value
// stored as a heap so stored dynamicaaly and cannnot return it 
fn get_full_name() -> &str{
    "john Doe"
}


fn get_fulls_name() -> & 'static str{ //this say that this string needs to live on for the entirely of the application life time 
    "john Doe" // i need to store this in a place in the heap that live on as long as the application running
}
//work correctly
// fn get_full_name() -> String{
//     "john Doe".to_string()
// }

// fn get_random_name(a: &str,b:&str)->&str{
//    a 
// }
//coorect one
//fn get_random_name<'l1,'l2>(a: &'l1 str,b:&'l2 str)->& 'l1 str{  //l2 lives as long as l1 lives
    fn get_random_name<'l>(a: &'l str,b:&'l str)->& 'l str{  //both have the same lifetime
    a // we cant change to b as l1 is value of a in case of l1 and l2
 }

 //missing lifetime annotation in struct 

//  struct Person{
    //name : &str, //Error 
 //}

//  struct Person <'a>{
//     name:&'a str,

//  }
 
// Define the struct with a generic lifetime parameter
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

// Implement methods for the struct using the same lifetime parameter
impl<'a> Person<'a> {
    // Returns a slice of the first character of first_name
    fn first_char_of_first_name(&self) -> &str {
        &self.first_name[0..1]
    }
}

//rust automoticall assign lifetime operators to both parameters
 fn get_first_name(full_name:&str)->&str{
    full_name
 }
enum Animal <'a>{
    Dog {name:&'a str},
}
 //Rules for life time in rust 
 //1 if you have 2 parameters to your function and their refrences the compiler assig a lifetime operator to ach one of them if you 
//2 if you have a single input lifetime automatically assigned to all outputs
//if you have a refrence to self or refrence to mutable self lifetime is assigned to output
pub fn run(){

let name = get_random_name("john" , "doe");
println!("{} , name");
}