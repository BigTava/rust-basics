use std::io;
use rand::prelude::*; // prelude stands for most common functions

fn main() {
    // ownership();
    // ref_ownership();
    // reference();
    // slices();
    // challenge_references();
    // input();
    // crates();
    // challenge_modules();
    challenge_modules_solution();
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


