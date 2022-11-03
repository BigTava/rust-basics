use std::io;
use rand::prelude::*; // prelude stands for most common functions
use std::env;
use std::fs;
use std::io::prelude::*;
use std::mem;

fn main() {
    // ownership();
    // ref_ownership();
    // reference();
    // slices();
    // challenge_references();
    // input();
    // crates();
    // challenge_modules();
    // challenge_modules_solution();
    // cmd();
    // read_files();
    // write_files();
    // challenge_io_solution();
    // create_struct();
    // create_tuple_struct();
    // challenge_structs();
    // generic();
    // generic_function();
    // box_data_type();
    challenge_generic();
}

/* GENERIC TYPES */
#[derive(Debug)] // allows printing object
struct GenericRectangle<T, U> {
    width: T, 
    height: U
}

impl<T,U> GenericRectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width 
    }
}

impl GenericRectangle<u8, u8> {
    fn get_perimter(&self) -> u8 {
        2*self.width+2*self.height
    }
}

fn generic() {
    let rect = GenericRectangle {
        width: 1u8,
        height: 3u8
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimiter is {}", rect.get_perimter());
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a 
    } else {
        b
    }
}

fn generic_function() {
    println!("biggest is {}", get_biggest(1.2,2.1));
}

fn box_data_type() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 8892834.0
    };
    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    println!("boxed_vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle)); // add *

    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));
}

fn sum_boxes<T: std::ops::Add<Output=T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a+*b)
}

fn challenge_generic() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);

    println!("Tests passed!");
}

/* STRUCTS */
#[derive(Debug)] // allows printing object
#[derive(Clone)] // allows cloning object
struct Shuttle {
    name: String, 
    crew_size: u8, 
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant:0.0
        }
    }
}



fn create_struct() {
    /*let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };*/
    let mut vehicle = Shuttle::new("Endeavour2");

    /*let mut vehicle2 = Shuttle {
        ..vehicle.clone()
    };*/
    let mut vehicle2 = Shuttle::new("Discovery");

    vehicle.crew_size = 6;
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}
struct Color(u8,u8,u8); //RGB
struct Point(u8,u8,u8); // XYZ

fn create_tuple_struct() {
    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);

    let coord = Point(4,5,6);
    let y = get_y(coord);
    println!("y is {}", y);
}

fn get_y(p: Point) -> u8 {
    p.1
}

struct Rectangle {
    width: f64, 
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.width*self.height
    }

    fn scale(&mut self, scaler: f64) {
        self.width *= scaler;
        self.height *= scaler;
    }

    fn new(_width: f64, _height: f64) -> Rectangle {
        Rectangle {
            width: _width,
            height: _height,
        }
    }
}

fn challenge_structs() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}

/* INPUTS AND OUTPUTS */
fn cmd() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    let arg2 =  env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}

fn read_files() {
    let contents = fs::read_to_string("./src/assets/planets.txt").unwrap();
    println!("contents is {}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }

    let contents = fs::read("./src/assets/planets.txt").unwrap();
    println!("contents is {:?}", contents);
}

fn write_files() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decate\n");
    speech.push_str("And do the other things,\n");
    speech.push_str("not because theu are easy,");

    fs::write("./src/assets/speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("./src/assets/planets.txt").unwrap();
    file.write(b"\nPluto");
}

fn challenge_io() {
    if env::args().len() != 3 {
        println!("{}", env::args().len());
        println!("Program requires 3 arguments.");
        return;
    }

    let file =  env::args().nth(1).unwrap();
    let name =  env::args().nth(2).unwrap();
    let mut found = false;

    let contents = fs::read_to_string(format!("./src/assets/{}", file)).unwrap();
    for line in contents.lines() {
        if name == line {
            println!("Found");
            found = true;
            return;
        }
    }

    if found == false {
        println!("Not Found");
    }
}

fn challenge_io_solution() {
    if env::args().len() < 2 {
        eprintln!("Program requires two arguments: <file path> <search name>");
        std::process::exit(1);
    }

    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line == search_name {
            println!("{} did walk on the Moon!", search_name);
            return;
        }
    }

    println!("{} did NOT walk on the Moon... YET!", search_name);
}

/* MODULES */
fn input(){
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);
}

fn crates() {
    // let number = rand::random::<f64>();
    let number = random::<f64>();
    println!("number is {}", number);

    let number = thread_rng().gen_range(1..11);
    println!("number is {}", number);
}

fn challenge_modules() {
    let number = thread_rng().gen_range(1..3);

    let mut buffer = String::new();
    println!("Enter your guess:");
    io::stdin().read_line(&mut buffer);
    let mut guess: i32 = buffer.trim().parse().unwrap();

    while guess != number {
        println!("Incorrect. Enter another guess:");
        buffer = String::new();
        io::stdin().read_line(&mut buffer);
        guess = buffer.trim().parse().unwrap();
    }

    println!("Correct!");
}

fn challenge_modules_solution() {
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("I'm thinking of a number between 1 and 100..");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input line.");
        let guess: u32 = guess.trim().parse().expect("Failed to parse the guess");

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }
}



/* REFERENCES */
fn trim_spaces(s: &str) -> &str{
    let mut last_front_space: usize = 0;
    for (index, character) in s.chars().enumerate() {
        if !character.is_ascii_whitespace() {
            last_front_space = index;
            break;
        }
    }

    let mut first_rear_space: usize = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if !character.is_ascii_whitespace() {
            first_rear_space = s.len() - index;
            break;
        }
    }
    
    &s[last_front_space..first_rear_space]
}

fn challenge_references()  {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " x🚀xx ";
    assert_eq!(trim_spaces(test7), "x🚀xx");
    println!("Tests passed!");
}

fn ref_ownership() {
    let mut rocket_fuel = String::from("RP-1");
    let length = ref_process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn ref_process_fuel(propellant: &mut String) -> usize {
    println!("processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}

fn slices() {
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    // should be careful about emojis or special characters that span multiple bytes
    let last_word = &message[15..];
    println!("last_word is {}", last_word);

    let planets = [1,2,3,4,5,6,7,8];
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are {:?}", inner_planets);

    let first_word = get_first_word(&message[10..]);
    // let first_word = get_first_word(&message[10..]);
    // impossible to pass slice of message as a reference slice is different from a string (it borrows from the heap)
    // a reference string borrows from a variable  that owns a string in the heap
    // it works if the argument in get_first_word is s: &str
    println!("first_word is {}", first_word);
}

fn reference() {
    let rocket_fuel = produce_fuel();
    println!("rocket_fuel is {}", rocket_fuel)
}

fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}

/* OWNERSHIP */
fn ownership() {
    let mut message = String::from("Earth");
    println!("{}", message);
    message.push_str(" is home");
    println!("{}", message);

    // Resources can only have one owner at a time 
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        // outer_planet = inner_planet;
        outer_planet = inner_planet.clone();
        inner_planet.clear();

        // value borrowed here after move
        // println!("inner_planet is {}", inner_planet);
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);

    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet is {}", inner_planet);
    }
    // Integers don't need to be cloned
    println!("outer_planet is {}", outer_planet);

    let rocket_fuel = String::from("RP-1");

    // 1st passes ownership of the string to process_fuel function
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    // 2nd transfers ownership back to the main function
    propellant
}


