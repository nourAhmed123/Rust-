//Import to can use hashmap
use std::collections::HashMap;

#[derive(Hash,Eq,PartialEq,Debug)]
struct Person{
name:String,
age:u8,
}
pub fn run() {
    let mut values: HashMap<&str, &str> = HashMap::new();
    values.insert("foo", "bar");
    println!("{:?}", values);
    // values.remove("foo");
    // println!("{:?}",values);
    if values.contains_key("name") {
        println!("name exists")
    } else {
        println!("No name")
    }

    //Safely reading values from hashmap
    match values.get("foo") {
        Some(value) => println!("{}", value),
        None => println!("Not found"),
    }
    //ITERATING OVER KEYS AND VALUES
    for (&k, &v) in &values {
        println!("{} {}", k, v);
    }
    //Retrieving an entry (rarelu used)
    let entry = values.entry("foo");
    let mut values2: HashMap<&str, &str> = HashMap::new();
    values2.insert("name", "nour ahmed");
    values2.entry("name").or_insert("nour ahmed");
    //inserting custom structs into hashmap 
    let mut persons: HashMap<Person, &str> = HashMap::new();

}
