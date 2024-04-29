//Tuples group together values of different types
//Max 12 elements
//Tuples doesnot have field names 
//Tuple struct
//accessing tuples with dot notation 
struct Point (f64,f64,f64);
fn get_values()->(String,String,i32){
    ("hello".to_string(),"world".to_string(),30)
}
pub fn run(){
let point=Point(0.0,1.0,2.0);
let person:(&str,&str,i8)=("Brad","Mass",37);
println!("{} is from {} and is {}",person.0,person.1,person.2);
// let values = ("hello","world",30);
// let(_,_,age)=values;
let (hello,_,_)=get_values();

}