//optionals can either have a value or not

pub fn run() {
    let value = Some(10);
    let name: Option<&str> = None; //Some("john Doe")
    //unwrap options safely
    match name {
        Some(name) => println!("Hello,{}", name),
        None => println!("There is no name"),
    }
    //unwrap unsafely
    // let unwrapped_name = name.expect("name wasnot provided");
    // force unwrap not a good idea
    //let unwrapped_name = name.unwrap();
    // println!("Name is {}", unwrapped_name);

    //Mutating option values
    let mut age: Option<i8> = Some(20);
    match age.as_mut(){
        Some(age)=>*age+=1,
        None=>{}
    }
    println!("Age is {}", age.unwrap());

    //unwrap multiple optionals with tuples 
    let mut age1: Option<i8> = Some(20);
    let mut age2: Option<i8> = Some(25);
    let mut age3: Option<i8> = Some(35);
  
    if let (Some(age_1),Some(age_2),Some(age_3))=(age1,age2,age3){
        println!("{}",age_1+age_2+age_3);
        println!("{} {} {}",age_1,age_2,age_3);
    }

    //unwrap with default value 
    let name: Option<&str>=None;
    //let unwrapped=name.unwrap_or("john Doe");
    let unwrapped=name.unwrap_or_else(||{
        //do some work
        "John Doe"
    });
    println!("name is {}",unwrapped);
    //check if option is some or None
   if name.is_some(){
    println!("There is a value")
   }else{
    println!("No value");
   }

    //unwrap with default value 
   let ages: Option<&i32>=None;
   let age=age.unwrap_or_default();
   println!("{}",age);

}
