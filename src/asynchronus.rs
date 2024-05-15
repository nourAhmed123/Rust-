use std::fmt::format;

use futures::executor::block_on; //This function will block the caller until the given future has completed.
use tokio::time::{sleep,Duration};
use futures::Future;


//Asyn/Await
async fn get_name()->String{
"john".to_string()
}

async fn call_api_one()-> String {
sleep(Duration:: from_secs(1)).await;
"one".to_string()
}

async fn call_api_two()-> String {
    sleep(Duration:: from_secs(1)).await;
    "two".to_string()
    }
    fn call_api_test()->impl Future<Output = String>{
        async{

            sleep(Duration:: from_secs(1)).await;
            "one".to_string()
        }
    }

    fn call_api_test2()->impl Future<Output = String>{
        async{

            sleep(Duration:: from_secs(1)).await;
            "two".to_string()
        }
    }

    fn get_async_name() -> impl Future<Output = String>{
     let name="john Doe".to_string(); 
     async move{
        format!("Hello {} Doe",name)
     }
    }
  
#[tokio::main]
pub async fn run (){
// let name = get_name();
// println!("Hello , {}",name); //will give error
let name =block_on(get_name()) ; //this way doesnot need async in the main function 
println!("Hello , {}",name); 
let one = call_api_one().await;
println!("{}",one);
let two =call_api_two().await;
println!("{}",two);
let result = get_async_name().await;
println!("{}", result);



}