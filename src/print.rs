pub fn run(){
    //print to console
    println!("Hello world from print.rs,");

    //Basic Formatting
    println!("{} is from {}","Brad","Mass");

    //Positional arguments
    println!("{0} is from {1} and {0} likes to {2}","Brad","Mass","code");
    
    //Named Arguments
    println!("{name} like to play {activity}", name="john", activity="baseball");

//Placeholder traits
println!("Binary: {:b} Hex: {:x} octal: {:o}", 10,10,10);
//placeholder for debug trait
println!("{:?}",(12,true,"hello"));
//Basic math
println!("10+10={}",10+10);

}