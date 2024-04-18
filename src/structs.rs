//Traditional struct
// struct Color{
//   red: u8 , //because rgb from 0 -255
// green:u8,
// blue:u8,

// }
//Tuple struct
// struct Color(u8,u8,u8);

struct Person1{
    name : String,
    age: u8
}

fn create_person(name: String , age :u8){
 let person = Person1{
  name,
  age,
 };   
}
struct Person {
    first_name: String,
    last_name: String,

}

impl Person {
    //construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color{
    //     red:255,
    //     green:0,
    //     blue:0
    // };
    // println!("Color: {} {} {}",c.blue,c.green,c.red);

    // let mut c = Color(255,0,0);
    // println!("color: {} {} {} ",c.0,c.1,c.2);
    let person = Person1{
        name: "Nour".to_string(),
        age:30
    };
   println!("{} is {} years old",person.name,person.age);
    let mut p = Person::new("Marry", "Doe");
    p.set_last_name("williams");
    println!("person{}", p.full_name());
    println!("person Tuple{:?}", p.to_tuple());
    //println!("person {} {}", p.first_name, p.last_name);
}
