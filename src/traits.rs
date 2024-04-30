// Traits are shared functionality
use std::fmt;
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}
trait HasFullName {
    fn full_name(&self) -> String;
}

trait HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

// trait HasFullName
// where
//     Self: HasName,
// {
//     fn full_name(&self) -> String {
//         format!("{} {}", self.first_name(), self.last_name())
//     }
// }

fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name());
}
//Trait bound syntax

fn print_details<T: HasFullName + CanRun>(value: &T){
    println!("{}", value.full_name());
    value.run();
}
//both works the same
// fn print_details<T>(value: &T)
// where
//     T: HasFullName + CanRun,
// {
//     println!("{}", value.full_name());
//     value.run();
// }
//conformance to multiple traits
trait CanRun {
    fn run(&self);
}
impl CanRun for Person {
    fn run(&self) {
        //todo
    }
}
impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} , {}", self.first_name, self.last_name)
    }
}

trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect(); //collect turn a map of string slices into a vector of string slices
        Person {
            first_name: parts[0].to_string(), //vector first component
            last_name: parts[1].to_string(),  //vector second component
            age: 30,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} is {} years old",
            self.first_name, self.last_name, self.age
        )
    }
}
impl HasName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }
}
pub fn run() {
    let person = Person {
        first_name: "john".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };
    println!("{:?}", person);
    //passing a string slice to a new function that specified on a trait
    let person2 = Person::new("Nour Ahmed");
    println!(
        "First name = {} , last name = {} , age = {}",
        person2.first_name, person2.last_name, person2.age
    );
    //fmt
    println!("{}", person2);
    //traits as parameters
    print_full_name_and_age(&person);
    //Hasfull Name
    let full_name=person.full_name();
    println!("{}",full_name);
}
