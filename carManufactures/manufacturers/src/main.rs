#![deny(clippy::all)]
use std::env; //for reading arguments
const API_URL: &str ="https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";
struct Manufacturer<'a>{
 name: Option<&'a str>,
 common_name:Option<&'a str>,
 country:Option<& 'a str>,
}
// Create contains trait to search for specific field
trait Contains{
    fn contains(&self , needle: &str)->bool;
}
impl<'a> Contains for Manufacturer<'a>{
    //Making sure every manufacturer has a function called contains to can search with the needle
    fn contains(&self , needle: &str)->bool{ 
        self.name.unwrap_or_default().contains(needle)
        || self.common_name.unwrap_or_default().contains(needle)
        ||self.country.unwrap_or_default().contains(needle)
    }
}
//Description function to can print the result of search
impl<'a> Manufacturer<'a>{
    fn description(&self)->String{
        let name=self.name.unwrap_or_default();
        let common_name=self.common_name.unwrap_or_default();
        let country=self.country.unwrap_or_default();
        format!(
            "\tName:{}\n\tCommon Name:{}\n\tCountry:{}",name,common_name,country
        )
    }
}
#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> { //this function will return a future
let args: Vec<String>=env::args().collect(); //Read arguments
//ensure that we have at least 2 arguments 
if args.len()<2{
    println!("usage: {} <search term",args[0]);
    return Ok(());
}
//Store keyword into a variable 
let keyword = &args[1];
let client = reqwest :: Client::new();
let res = client
.get(API_URL)
.send().await?
.json::<serde_json::Value>()
.await?; //Make sure await is optional 
//Grab the json objects
let manufacturer_json = res
.as_object()
.unwrap()
.iter()
.find(|(key, _)| key == &"Results") 
.unwrap()
.1 //Value of Results
.as_array()
.unwrap() //Make sure value is an array 
.iter();
//Parse all the manufacturers
let manufacturers = manufacturer_json.map(|manufacturer_json|{
let obj=manufacturer_json.as_object().unwrap();
let country = obj.get("Country").unwrap().as_str();
let common_name = obj.get("Mfr_CommonName").unwrap().as_str();
let name = obj.get("Mfr_Name").unwrap().as_str();

Manufacturer{
    name,
    common_name,
    country,
}
  });
       //Find the relevant manfucature
       let found_manufacturers= manufacturers
       .filter(|manufacturer|manufacturer.contains(keyword)).collect::<Vec<_>>();
   if found_manufacturers.is_empty(){
    println!("no manufacturers found");
   }else{
    println!("Found {} manufacturers:",found_manufacturers.len());
    for (index, man) in found_manufacturers.iter().enumerate() {
        println!("manufacturers # {}", index + 1);
        println!("{}", man.description());
    }
   }
    Ok(())
}
