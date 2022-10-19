fn main() {
    ownership();
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
