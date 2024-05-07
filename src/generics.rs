use std::ops::AddAssign;
#[derive(Debug)]
struct Point<T> {
    //make this struct genereic accept  any type
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn move_offset(&mut self, x: T, y: T)
    where
        T: AddAssign,
    {
        self.x += x;
        self.y += y;
    }
}
impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

//Another example for implementing specific trait on generic types
trait CanRun {
    fn run(&self);
}
trait CanWalk {
    fn walk(&self);
}
impl<T: CanRun> CanRun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run();
        }
    }
}
impl<T: CanWalk> CanWalk for Vec<T> {
    fn walk(&self) {
        for item in self {
            item.walk();
        }
    }
}

struct Person {
    name: String,
}
impl CanWalk for Person {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}
impl CanRun for Person {
    fn run(&self) {
        println!("{} is running", self.name);
    }
}
//struct elephoant that's can't run but able to walk
struct Elephant {
    name: String,
}
impl CanWalk for Elephant {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}
pub fn run() {
    let mut p = Point { x: 1.0, y: 2.0 };
    //let p3 = Point{x:"Foo",y:"Bar"};
    let p4 = Point { x: 3.0, y: 4.0 };
    //p.move_offset(3.0,4.0);
    println!("{:?}", p);
    p += p4;
    println!("{:?}", p);

    let p1 = Point { x: 0.0, y: 7.0 };
    let p2 = Point { x: 0.0, y: 7.0 };
    if p1 == p2 {
        println!("p1 and p2 are equal");
    } else {
        println!("they are not equal")
    }

    //Another example
    let people = vec![
        Person {
            name: "john".to_string(),
        },
        Person {
            name: "jane".to_string(),
        },
        Person {
            name: "joe".to_string(),
        },
    ];
    people.run();
    people.walk();
    let elephants = vec![
        Elephant {
            name: "Elephant1".to_string(),
        },
        Elephant {
            name: "Elephant2".to_string(),
        },
    ];
    //elephants.run(); will give error as elephoane dont conform to can run trait
    elephants.walk();
}
