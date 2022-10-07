// cargo run --bin quux
fn main() {
    /* Arrays */
    let letters = ["a", "b", "c"];
    let first_letter = letters[0];
    println!("first letter is {}", first_letter);

    let mut mut_letters = letters;
    mut_letters[0] = "x";
    println!("first letter is {}", mut_letters[0]);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len() - 1;
    println!("last number is {}", numbers[index]);
    
    /* Multidimensional arrays */
    let parking_lot = [[1,2,3], [4,5,6]]; // inner arrays need to have same number of elements
    let number = parking_lot[1][2];
    println!("number is {}", number);

    let _garage = [[[0; 100]; 20]; 5];  

    /* Tuples */
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}", first_item);

    let(_a,b,_c) = stuff;
    println!("b is {}", b);
}