//iter cannot be doubled consumed
pub fn run(){
    let values=vec![1,2,3,4,5];
    let iter=values.iter();
    let sum1:i32=iter.sum();
   // let sum2:i32=iter.sum(); error mynf3sh astkhdm iter marten
// let names = vec!["john","jane,Mary","Bob","Tom"];
// for name in names.into_iter(){
//     println!("{}",name)
// }
// for name in names.iter(){
//     println!("{}",name)
// }
let names = vec!["john", "jane", "Mary", "Bob", "Tom"];
// First iteration using borrowed elements
for name in names.iter() {
    println!("{}", name);
}

// Consuming the vector in the second iteration
// for name in names.into_iter().filter(|name|name.len()==3) {
    for name in names.into_iter() {
        if name.len()!=3{
           //continue;
           break;
        }
    println!(" in{}", name);
}
}