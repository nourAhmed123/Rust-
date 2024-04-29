fn get_user_name()->Result<String,()>{
     Ok("John".to_string())
   // Err(())
}

// fn get_first_name()->Result<String,()>{
//     Ok("john".to_string())
// }

// fn get_last_name()->Result<String,()>{
//    // Err(())
//    Ok("Doe".to_string())
// }

// fn get_full_name()->Result<String,()>{
//     //Err(())
//     let first_name=get_first_name()?;
//     let last_name=get_last_name()?;
//     Ok(format!("{}{}",first_name,last_name))
// }


fn get_first_name()->Result<String,String>{
  // Ok("john".to_string())
  Err(("I dont know").to_string())
}

fn get_last_name()->Result<String,String>{
  // Err(())
  Ok("Doe".to_string())
}

fn get_full_name()->Result<String,String>{
     //Err(())
     let first_name=get_first_name()?;
     let last_name=get_last_name()?;
     Ok(format!("{}{}",first_name,last_name))
 }
 
pub fn run(){
// create a simple result
// Box: Refrence to memory in the heap
// let value: Result<&str,Box<dyn std::error::Error>>= Ok("Hello World!");
// match value{
//     Ok(value) => println!("{}",value),
//     Err(error) => println!("{}",error),
// }

let value: Result<&str,()>=Err(()); //Ok("Hello World")
match value{
    Ok(value) => println!("{}",value),
    Err(_) => println!("Some Error occured"),
}
// let value2: Result<&str,()>=Err(());
// let unwrapped=value.expect("I was expecting a username");
 let user_name= get_user_name().expect("Failed to get username");
println!("Hello,{}",user_name);
//check ok and err with if-statnment
let is_ok = get_user_name().is_ok();
let is_err=get_user_name().is_err();
println!("{} {}",is_ok,is_err);
let full_name=get_full_name();
// match full_name{
//     Ok(name) => println!("Hello , {}!",name),
//     Err(_)=>println!("Error!"),
// }
//Map ok in Result
//let length = full_name.map( |s| s.len());
//println!("{:?}" , length);  // Uses debug formatting as map returns Result<usize, ()>
//To get length of error
let error_length=full_name.map_err(|e| e.len());
println!("{:?}" , error_length);
}