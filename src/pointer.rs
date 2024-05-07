//Box ->  A pointer type that uniquely owns a heap allocation of type T.
//allow you to store data on the heap rather than the stack.

//implement your own box
use std::ops::Deref;

//Define Your Box
struct BoxedValue<T> {
    value: T,
}
impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}
impl<T> Deref for BoxedValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn print_integer(value: &i32) {
    println!("{}", value);
}

//RC ->  ‘Reference Counted’.
//Disallows mutation of the wrapped value
// keeps track of the number of references to a value to determine whether or not the value is still in use
//Mutability not allowed with rc

use std::rc::Rc;

//cell to mutate
//cell allows internal mutability (unsafe)
use std::cell::Cell;
struct Person{
    name: String,
    age:Cell <u8>,
}
impl Person{
    fn increment_age(&self)->u8{
        self.age.set(self.age.get()+1);
        self.age.get()
    }
}

//ref cell 
use std ::cell::RefCell;
pub fn run() {
    let age = Box::new(22);
    let twice = *age * 2;
    println!("{}", twice);
    let age2 = BoxedValue::new(22);
    //trying to derefrence will give error Without implementing deref

    println!("value is {}", *age2); //* gives you the Actual value */
                                    //another option .deref gives you refrence to the actual value
    let actual_age = age2.deref();
    println!("{}", actual_age);
    let ref_to_value = age2.deref();
    let other = *(age.deref()); //
    let value = BoxedValue::new(10);
    print_integer(&value);

    //RC
    let array = vec!["john".to_string(), "jane".to_string()];
    let rc = Rc::new(array);
    //cloning rc create a new rc project
    let rc2=rc.clone();
    //same 
    let rc3=Rc::clone(&rc);
    //weak refrence to the vector 
    let weak = Rc::downgrade(&rc);
    drop(rc); 
    //crash when upgrading and unwrapping weak refrence
//let value = weak.upgrade().unwrap();
// println!("{:?}",value);

match weak.upgrade(){
    Some(rc)=>println!("{:?}",rc), //IF WE COMMENTED DROP RC IT WILL GIVE THE RC 
    None=>println!("none")
};
//cell
let person = Person{
    name:"John".to_string(),
    age: Cell::new(20)
};
let new_age=person.increment_age();
let person_age=person.age.get();
println!("{}",new_age);
println!("{}",person_age);
// RefCell
let ref_cell=RefCell::new(vec![1,2,3]);
let mut mut_ref=ref_cell.borrow_mut();let len = ref_cell.borrow().len();
let len = ref_cell.borrow().len();
mut_ref.push(100);
//println!("length = {}",len); //will panick
}
