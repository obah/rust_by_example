use std::fmt;

struct List(Vec<i32>);

//unit struct - use for generics
struct Unit;

//tuple struct - named tuples
struct Pair(i32, f32);
//regular struct
struct Point {
    x: u32,
    y: u32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Person {
    name: String,
    height: f32,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{0}:{1}", count, v)?;
        }

        write!(f, "]")
    }
}

enum WebEvent {
    //variant can be unit
    PageLoad,
    PageUnload,
    //like tuple struct
    KeyPress(char),
    Paste(String),
    //regular struct
    Click { x: i64, y: i64 },
}

enum Role {
    Student,
    Teacher,
}

//enum can be like struct
enum Color {
    Black = 0x000000,
    White = 0xFFFFFF,
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(_char) => println!("{_char} was pressed"),
        WebEvent::Paste(_string) => println!("{_string} was pasted"),
        WebEvent::Click { x, y } => println!("coordinates {x} and {y} were clicked."),
    }
}

fn printer() {
    let v = List(vec![1, 2, 3]);

    println!("{}", v);

    println!(
        "This is the first element: {0} and second: {1} and first again: {0}",
        "1st", "2nd"
    );

    println!(
        "{one} is to err, {two} is to run with {three}",
        one = "Something",
        two = "Everything",
        three = "Nothing"
    );

    println!("{number:0>5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let pi = 3.141592;
    println!("Pi is roughly `{pi:.3}`");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct {:#?} won't print...", Structure(3));
    // TODO ^ Try uncommenting this line
}

fn binary_search(arr: &[u32], num: u32, min: usize, max: usize) -> usize {
    let index: usize = (min + max) / 2;

    if arr[index] == num {
        return index;
    } else if index == 0 {
        return 0;
    } else if arr[index] > num {
        let new_max = index - 1;

        binary_search(arr, num, min, new_max)
    } else {
        let new_min = index + 1;

        binary_search(arr, num, new_min, max)
    }
}

fn primitives() {
    let tuple_1 = (43u8, (50i32, false), true, 4.3f32, 2555i16, "hey", 'i');

    println!(
        "this is the first element in the tuple element in the tuple: {}",
        tuple_1.1 .0
    );

    println!("this is the tuple in the tuple: {:?}", tuple_1.1);

    let (a, b) = tuple_1.1;

    println!("{:?} & {:?}", a, b);

    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));
}

fn rect_area(_rect: Rectangle) -> u32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = _rect;

    let side1 = y2 - y1;
    let side2 = x2 - x1;

    side1 * side2
}

fn main() {
    primitives();
    let primes: [u32; 25] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    let max1 = primes.len();

    let result = binary_search(&primes, 67, 0, max1);

    println!("found number in {result}");

    printer();

    let _unit = Unit;

    let _pait = Pair(10, 10.0);

    let _point_1 = Point { x: 0, y: 0 };
    let _point_2 = Point { x: 15, y: 8 };

    let _rect1 = Rectangle {
        top_left: _point_1,
        bottom_right: _point_2,
    };

    let _rect1_area = rect_area(_rect1);

    let name = String::from("James");
    let height = 6.1;

    let james = Person { name, height };

    println!("New created struct person has {:?}", james);

    println!("area of the rectange is {_rect1_area}");

    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let pressed = WebEvent::KeyPress('c');
    //to_owned() creates an owned string from a string slice
    let paste = WebEvent::Paste("this is fun".to_owned());
    let click = WebEvent::Click { x: 2, y: 2 };

    inspect(load);
    inspect(unload);
    inspect(pressed);
    inspect(paste);
    inspect(click);

    //use use to make an enum available without manual scoping
    use crate::Role::*;
    let role = Student;

    println!("The color is {}", Color::White as i32);

    let test_var = "First";

    {
        //shadowing
        let test_var = "shadowed";

        println!("current test_var is {:?}", test_var);
    }
    println!("first test_var outside block is {:?}", test_var);

    let test_var = "second";

    println!("second shadowed test_var is {:?}", test_var);

    let float_1 = 65.4344;
    let int_1 = float_1 as u8;
    let char_1 = int_1 as char;

    println!("float value is {float_1}, u8 value is {int_1} and character is {char_1}");

    // let char_2 = float_1 as char; // doesnt work because only u8 can be casted to char

    println!(" 128 as a i16 is: {}", 128 as i16);

    // println!(" 128 as a i8 is : {}", 128 as i8); wont work

    // println!("1000 as a u8 is : {}", 1000 as u8); //wont work also
}
