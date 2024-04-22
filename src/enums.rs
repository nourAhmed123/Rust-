#![deny(clippy::all)]
//Enums are types which have a few definite values
//Enumerations are useful for related objects
#[derive(PartialEq)]
enum AnimalType {
    //Name of enumerations pascal case
    Dog,
    Cat,
    Rabbit,
}
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}
struct Size {
    width: f32,
    height: f32,
}
enum Shapes {
    // Circle {radius:f64 , center:(f64,f64)},
    // Rectangle {width:f64,height:f64},
    Circle(f32, f32, f32),
    Rectangle(f32, f32, Size),
}
impl Shapes {
    fn area(&self) -> f32 {
        match self {
            Shapes::Rectangle(x, y, Size) => Size.width * Size.height,
            Shapes::Circle(x, y, radius) => 3.14 * radius * radius,
        }
    }
}
enum Pet{
    Cat{name:String},
    Dog{name:String},
}
fn move_avatar(m: Movement) {
    //perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}
pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Right;
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
    let fluffy = AnimalType::Dog; //Instance of enumeration
    if fluffy == AnimalType::Dog {
        println!("Fluffy is a dog!");
    };
    match fluffy {
        //AnimalType::Dog=> println!("Woof!"),
        AnimalType::Cat => println!("Meow!"),
        AnimalType::Rabbit => println!("Hoot!"),
        _ => println!("Something else"), //Default case
    };
    // let rectangle = Shapes::Rectangle{
    //     width:3.0,
    //     height:4.0,
    // };
    // if let Shapes::Rectangle{width,height}=rectangle{
    //     println!("width = {} , height = {} ",width,height);
    // }
    // match rectangle {
    //     Shapes::Rectangle{width,height}=>{
    //         println!("{}",width*height);
    //     }
    //     _=>println!("Not a rectangle"),
    // }

    let rectangle = Shapes::Rectangle(
        1.0,
        2.0,
        Size {
            width: 3.0,
            height: 4.0,
        },
    );
    // if let Shapes::Rectangle(x, y, Size { width, height }) = rectangle {
    //     println!("{} {} {} {}", x, y, width, height);
    // } else {
    //     println!("Not a rectangle")
    // }
   let area = rectangle.area();
    println!("area is {}",area);
    let fluffy=Pet::Cat{name:"fluffy".to_string()};
    let name = match fluffy {
        Pet::Cat {name}=>name,
        Pet::Dog {name}=>name,
    };
    println!("Hello,{}",name)
}
