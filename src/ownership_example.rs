//Rust Drop variables that go out of scope
//Traits : shared behaviour or feature
//Stack first in last out , last in first out
//Heap Dynamic memory Allocation
//Refrences Read only view of another variable (pointer)
//Refrences doesnot drop its values
//Refrences can be mutable and immutable
//you can have at most one mutable refrence at a time to prevent data races
//Data races  occur when multiple tasks or threads access a shared resource without sufficient protections, leading to undefined or unpredictable behavior.
//Dangling Refrence -> when a reference (such as a pointer or a variable in programming) refers to a memory location of a resource that has been deallocated or is no longer valid. This typically happens in situations where an object has been deleted or has gone out of scope, but there are still references to it.
//     fn get_name()-> &String {
// &"JOHN".to_string()
//     }
//     let n=get_name();

pub fn run() {
    fn greet(name: &String) {
        println!("HELLO,{}!", name)
    }
    fn empty_string(value: &mut String) {
        value.clear();
    }
    // let name1= String::from("john");
    // let name2= name1;

    // println!("Hello,{}",name1);
    // println!("Hello,{}",name2);  //Gives error

    let name1 = String::from("john");
    let name2 = &name1; //Refrence
    greet(name2);
//greet(name1);//error as the function expect string refrence (&String )not string
   let mut name=String::from("john");

   empty_string(&mut name);
    println!("Hello,{}", name1);
    println!("Hello,{}", name2);
    let age1 = 10;
    let age2 = age1;
    println!("you are {} years old ", age1);
    println!("you are {} years old ", age2); // no errors because it stores integers in stack  copying age1 in age 2
}
