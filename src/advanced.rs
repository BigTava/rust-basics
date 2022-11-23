use std::any;
use std::fmt;

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
    struct_lifetime_annotations();
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
