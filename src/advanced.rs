use std::any;
use std::fmt;
use std::fs;
use std::io;
use rand::prelude::*;
use std::collections::HashMap;

fn main() {
    // simple_trait();
    // derive_trait();
    // bounds_trait();
    // multiple_trait_bounds();
    // return_types();
    // challenge_traits()
    // borrow_checker();
    // lifetime_annotation();
    // lifetime_elision_rules();
    // struct_lifetime_annotations();
    // define_enums();
    // match_expression();
    // option();
    // matching_option();
    // if_let();
    // challenge_enums();
    // unrecoverable_errors();
    // result_enum();
    // recover_from_errors();
    // propagating_errors();
    // challenge_errors();
    vectors();
    hashmap();
    challenge_collections();
}

/* COLLECTIONS */
fn vectors() {
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("astronauts is {:?}", astronauts);

    let third = &astronauts[2];
    println!("Third is {}", third);

    let last = astronauts.pop();
    println!("last is {:?}", last);

    let popped = astronauts.get(2);
    println!("Third is {:?}", popped);
}

fn hashmap() {
    let mut missions_flown = HashMap::new();
    missions_flown.insert("Hadfield", 3);
    missions_flown.insert("Hurley", 3);
    missions_flown.insert("Barron", 0);
    println!("missions_flown is {:?}", missions_flown);

    missions_flown.insert("Barron", 1);
    missions_flown.entry("Stone").or_insert(2);
    let kayla = missions_flown.entry("Barron").or_insert(0);
    *kayla += 1;
    println!("missions_flown is {:?}", missions_flown);

    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {:?}", barron_missions);
}

fn challenge_collections() {
    // read file and build vector of individual words
    let contents = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Could not read file: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Program requires an argument: <file path>");
            std::process::exit(2);
        }
    };
    let all_words = contents.split_whitespace().collect::<Vec<&str>>();

    // count how many times each unique word occurs
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in all_words.iter() {
        *word_counts.entry(word).or_insert(1) += 1;
    }
    
    // determine the most commonly used word(s)
    let mut top_count = 0u32;
    let mut top_words: Vec<&str> = Vec::new();
    for (&key, &val) in word_counts.iter() {
        if val > top_count {
            top_count = val;
            top_words.clear();
            top_words.push(key);
        } else if val == top_count {
            top_words.push(key);
        }
    }

    // display results
    println!("Top word(s) occurred {} times:", top_count);
    for word in top_words.iter() {
        println!("{}", word);
    }
}

/* ERROR HANDLING */
fn unrecoverable_errors() {
    // panic!("Houston, we've had a problem!");
    let countdown = [5,4,3,2,1,0];

    for count in countdown.iter() {
        println!{"T-minus {}", count};
        let x = 1 / count; 
    }
}

fn result_enum() {
    let contents = fs::read_to_string("the_ultimate_question.txt").expect("Nobody knows the ultimate question!");
    println!("contents is : {:?}", contents);
}

fn recover_from_errors() {
    let result = fs::read_to_string("the_ultimate_question.txt");

    /*
    let contents = match result {
        Ok(message) => message,
        Err(error) => String::from("Nobody knows the ultimate question!")
    };

    println!("contents is : {:?}", contents);
    */

    let contents = match result {
        Ok(message) => message,
        Err(error) =>  match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found."),
            io::ErrorKind::PermissionDenied => String::from("Permission denied."),
            _ => panic!("Another type of error: {:?}", error)
        }
    };

    println!("contents is : {:?}", contents);
}

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(f1)?; // shorthand error handling

    // exactly the same as the previous
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e)
    };

    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn challenge_errors() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut buffer = String::new();
        let guess = match io::stdin().read_line(&mut buffer){
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(value) => value, // success
                Err(_) => {
                    println!("\nFailed to parse input. Guess again:");
                    continue
                }
            }
            Err(_) => {
                println!("\nFailed to read input. Guess again:");
                continue
            }
        };

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

fn propagating_errors() {
    let result = read_and_combine("./src/assets/planetsd.txt", "./src/assets/dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is...\n{}",s),
        Err(e) => println!("There was an error: {}",e),
    }
}

/* ENUMS */
#[derive(Debug)]
enum Shape {
    Circle(f64), 
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => r*2.0*std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c
        }
    }
}

fn define_enums() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("{} x {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c)
    }

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);
}

fn match_expression() {
    let my_number = 4u8;

    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("result is {}", result);
}

fn option() {
    let countdown = [5,4,3,2,1];
    let number = countdown.get(5);

    // creates problems. What is the option enum is none?
    // let number = number.unwrap() + 1;

    // use this
    let number = number.unwrap_or(&0) + 1;

    println!("number is {:?}", number);
}

fn matching_option() {
    let countdown = [5,4,3,2,1];
    let number = countdown.get(2);
    // let number = number.unwrap_or(&0) + 1;
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);
}

fn if_let() {
    let number = Some(13);

    if let Some(13) = number {
        println!("thirteen")
    }
}

#[derive(Debug)]
enum Location {
    Unknown, 
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        match self {
            Location::Unknown => println!("Unkown"),
            Location::Anonymous => println!("Anonymous"),
            Location::Known(lat, lon) => println!("{}, {}", lat, lon)
        }
    }
}

fn challenge_enums() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604133);
    address.display();

}

/* LIFETIMES */
// lifetime annotation
fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else {
        x
    }
}

fn lifetime_annotation() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}


fn borrow_checker() {
    let propellant;
    let rp1 = String::from("RP-1");
    {

        propellant = &rp1;

    }
    println!("propellant is {}", propellant);
}

fn get_first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }

    &s // no space found; input is a single word
}

fn lifetime_elision_rules() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}

struct Shuttle<'a> {
    name: &'a str
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn struct_lifetime_annotations() {
    let vehicle = Shuttle {
        name: "Endeavour"
    };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);
}

/* TRAITS */
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32
}

trait Description {
    fn describe(&self) -> String {
        String::from("an object flying through space!")
    }
}

impl Description for Satellite {
    
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying {} miles high with {} crew members aboard!", self.name, self.altitude, self.crew_size)
    }
}

fn simple_trait() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };

    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254
    };
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}

fn derive_trait() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42
    };
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}

fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {}", item, any::type_name::<T>());
}

fn bounds_trait() {
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]);
}

fn compare_and_print<T, U>(a: T, b: U) 
    where T: fmt::Display + PartialEq + From<U>,
          U: fmt::Display + PartialEq + Copy
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn multiple_trait_bounds() {
    compare_and_print(1.0, 1);
    // does not work
    // compare_and_print(1.0, "one");
    compare_and_print(1.1, 1);
}

fn get_displayable() -> impl fmt::Display {
    "thirtheen"
}
fn return_types() {
    println!("output is {}",get_displayable());
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.velocity)
    }
}

fn challenge_traits() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}
