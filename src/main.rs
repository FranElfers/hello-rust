fn main() {
    if_statements();
    infinite_loops();
    while_loops();
    for_loops();
    enums();
    constants();
    tuples();
    functions();
    code_blocks();
    shadowing();
    references();
    structs();
    tuple_structs();
    pass_by_reference();
    arrays();
    implementation();
    strings();
    implementing_traits();
    vectors();
    file_reading();
    command_line_arguments();
    file_writting();
    defining_traits();
    pattern_matching();
    read_user_input();
    hash_maps();
    random_numbers();
}

fn if_statements() {
    let a = 6.7;

    if a < 30.0 {
        println!("a is less than 30.0 {}", a);
    } else if a == 6.7 {
        println!("a is 6.7 {}", a);        
    } else {
        println!("a is greater than 30.0 {}", a);        
    }
}

fn infinite_loops() {
    let mut b = 0;
    loop {
        b += 1;

        if b == 7 {
            continue;
        }

        if b > 10 {
            break;
        }

        println!("b is {}", b)
    }
}

fn while_loops() {
    let mut c = 0;
    while c < 50 {
        c += 1;
        // "continue" & "break" work as well
    }
    println!("c is {}", c);
}

fn for_loops() {
    let range = 1..11;
    for i in range {
        println!("i is {}", i);
        // 11 non inclusive
        // "continue" & "break" work as well
    }

    let animals = vec!["Rabbit","Dog","Cat"];
    for a in animals.iter() {
        println!("a is {}", a)
    }
    for (index, a) in animals.iter().enumerate() {
        println!("index is {}, animal is {}", index, a)
    }
}

#[allow(dead_code)]
fn enums() {
    enum Direction {
        Up,
        Down,
        Left,
        Right
    }
    let player_direction:Direction = Direction::Up;
    match player_direction {
        Direction::Up => println!("We are heading up"),
        Direction::Down => println!("We are heading down"),
        Direction::Left => println!("We are heading left"),
        Direction::Right => println!("We are heading right")
    }
}

fn constants() {
    const MAXIMUM_NUMBER:u8 = 255;
    println!("Maximum number is {}", MAXIMUM_NUMBER);
}

fn tuples() {
	let tup: (i32, f64, u8) = (500, 3.5, 1);

	let (x,y,z) = tup;

	let a = tup.0;

	println!("X: {} Y: {} Z: {}", x,y,z);
	println!("A: {}", a);
}

fn functions() {
    let x = is_even(4);
    fn is_even(num:i32) -> bool {
        return num % 2 == 0
    }
    println!("x is {}", x);
}

fn code_blocks() {
    let mut x = 10;

    {
        x = 0;
        let y = 5;
    }
    println!("{}", x);
    // println!("{}", y); // throws error
}

fn shadowing() {
    let x = 10;
    {
        let x = 15; // now local var 
    }

    println!("{}", x); // 10
}

fn references() {
    // https://youtu.be/u4KyvRGKpuI
    let x = 10;
    let xr = &x;

    let mut y = 11;
    {
        let yr = &mut y;
        *yr = 12;
    }

    println!("x is {}, xr is {}, y is {}", x, xr, y);
}

fn structs() {
    struct Color {
        red: u8, // u8 supports 0-255, convenient for rgb
        green: u8,
        blue: u8
    }

    let mut bg = Color{ red: 255, green: 70, blue: 15 }; // also inmutable
    bg.blue = 54;

    println!("{},{},{}", bg.red, bg.green, bg.blue);
}

fn tuple_structs() {
    struct Color(u8, u8, u8);
    
    let mut red = Color(0, 0, 0); // also inmutable
    red.0 = 255;

    println!("red is {},{},{}", red.0, red.1, red.2);
}

fn pass_by_reference() {
    struct Color {
        red: u8, // u8 supports 0-255, convenient for rgb
        green: u8,
        blue: u8
    }

    let bg = Color{ red: 255, green: 70, blue: 15 }; // also inmutable

    fn print_color(c:&Color) {
        println!("Color is {},{},{}", c.red, c.green, c.blue);
    }

    print_color(&bg);
}

fn arrays() {
    let numbers = [1,2,3];
    println!("first value is {}", numbers[0]);

    for n in numbers.iter() {
        println!("{}", n);
    }

    let specific: [i32; 5] = [1,2,3,4,5]; // [type; length] (not actually required)

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }

    let default = [2; 100]; // a list of 2s with length of 100: [2,2,2,...]
}

fn implementation() {
    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        fn print_description(&self) {
            println!("Rectangle: {}x{}", self.width, self.height);
        }

        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }

    let my_rect = Rectangle { width: 10, height: 5 };

    my_rect.print_description();
    println!("Is a square: {}", my_rect.is_square())
}

fn strings() {
    let mut my_string = String::from("How's it going? My name is Dom.");

    println!("Is empty: {}", my_string.is_empty());
    println!("Length: {}", my_string.len());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!("Contains 'Dom'? {}", my_string.contains("Dom"));

    my_string.push_str(" Welcome");

    println!("New string {}", my_string);

    println!("After replace: {}", &my_string.replace("Dom", "Fran"));

    for section in my_string.split("?") {
        println!("Separated by ?: {}", section);
    }

    // form an array from splitted string
    let tokens: Vec<&str> = my_string.split("?").collect();

    // trim white spaces from edges
    let trimmed = my_string.trim();

    // first character
    my_string.chars().nth(0);

    {
        let text = String::from("The weather is\nnice");
        for line in text.lines() {
            println!("[{}]", line);
        }
    }
}

fn implementing_traits() {
    struct Person { 
        name: String,
        age: u8
    }

    impl ToString for Person {
        fn to_string(&self) -> String {
            return format!("My name is {} and I am {}.", self.name, self.age)
        }
    }

    let dom = Person { name: String::from("Domenic"), age: 21 };
    println!("{}", dom.to_string());
}

fn defining_traits() {
    struct Person {
        name: String,
        age: u8
    }

    trait HasVoiceBox {
        fn speak(&self);
        fn can_speak(&self) -> bool;
    }

    impl HasVoiceBox for Person {
        fn speak(&self) {
            println!("Hello, my name is {}", self.name);
        }

        fn can_speak(&self) -> bool {
            if self.age > 0 {
                return true;
            }
            return false;
        }
    }

    let person = Person {
        name: String::from("Bob"),
        age: 41
    };

    person.speak();
    println!("Can {} speak? {}", person.name, person.can_speak());
}

fn vectors() {
    let my_vector1:Vec<i32> = Vec::new();
    let mut my_vector = vec![1, 2, 3, 4]; // easy and common way

    my_vector.push(30);
    my_vector.remove(1); // removes second position

    for number in my_vector.iter() {
        println!("{}", number);
    }
}

use std::fs::File;
use std::io::prelude::*;

fn file_reading() {
    let mut file = File::open("info.txt").expect("Can't open file!");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Oops! Can't read the file");
    
    println!("File contents:\n\n{}", contents);
}

fn file_writting() {
    let mut file = File::create("info.txt").expect("Couldn't create file!");

    file.write_all(b"Welcome to dcode!")
        .expect("Cannot write to the file");
}

use std::env;

fn command_line_arguments() {
    let args: Vec<String> = env::args().collect();

    for argument in args.iter() {
        println!("{}", argument);
    }
}

fn pattern_matching() {
    let number = 8;

    match number {
        1 => println!("It's one"),
        2 => println!("There is two"),
        10 | 11 => println!("It's either ten or eleven"),
        3..=8 => println!("It's between 3 and 8"),
        _ => println!("It doesn't match")
    }
}

use std::io;

fn read_user_input() {
    let mut input = String::new();

    println!("Hey mate! Say something:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! You said {}", input);
        },
        Err(e) => {
            println!("Oops! Something went wrong, {}", e);
        }
    }
}

use std::collections::HashMap;

fn hash_maps() {
    // key => value
    let mut marks = HashMap::new();

    marks.insert("Rust programming", 96);
    marks.insert("Web development", 94);
    marks.insert("UX design", 75);
    marks.insert("Professional computing studies", 44);

    println!("Length of HashMap: {}", marks.len());

    match marks.get("Web development") {
        Some(mark) => println!("You got {} for Web dev!", mark),
        None => println!("You didn't study Web development")
    }

    marks.remove("UX design");

    for (key, value) in &marks {
        println!("For {} you got {}", key, value);
    }

    println!("Did you study C++? {}", marks.contains_key("C++")); // false
}

// install rand = "0.8.5"
use rand::Rng;
fn random_numbers() {
    let random_number = rand::thread_rng().gen_range(1..11); // from 1 to 10
    println!("Random number: {}", random_number);

    // Flip a coin
    let random_bool = rand::thread_rng().gen_bool(1.0/2.0); // 1 in 2 chances of true
    println!("Coin result {}", random_bool);
}






















