fn main() {
    // data_types();
    // basics();
    // funcions();
    // conditional_execution();
    loops();
}

/* PROGRAM FLOW CONTROL */
fn conditional_execution() {
    let x = 3;

    if x + 1 != 3 {
        println!("x + 1 is NOT 3!");
    }

    let y = 3;
    if x > y {
        println!("x is greater than y")
    } else if x < y { 
        println!("x is lower to y")
    } else {
        println!("x is equal to y")
    }

    let make_x_odd = true;
    let x = if make_x_odd {1} else {2};

    println!("x is {}", x);
}

fn loops() {
    let mut count = 0;

    // loop statement can be used to return a value
    let result = loop {
        count += 1;
        println!("count is {}", count);
        if count == 5 {
            break count;
        }
    };

    println!("After the loop!");
    println!("result is {}", result);

    count = 0;
    while count <10 {
        count += 2;
        println!("count is {}", count);
    }

    count = 0;
    let letters = ["a", "b", "c"];

    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }

    for item in letters {
        println!("for loop letter is {}", item);
    }

    for (index, &item) in letters.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == "e" {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }

    let mut matrix = [[1,2,3], [2,3,4],[5,6,7]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }

    /* CHALLENGE */
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = 0;
    let mut min: i32 = 0;
    let mut mean: f64 = 0.0;

    let mut count: i32 = 0;
    for number in numbers {
        count += 1;
        if number > max {
            max = number;
        } else if number < min {
            min = number;
        } 

        mean += number as f64;
    }

    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");

}

/* FUNCTIONS */
fn funcions() {
    say_a_number(13);
    say_the_sum(1,1);
    let result = square(3);
    println!("result is {:?}", result);

    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!")
} 

fn celsius_to_fahrenheit(c: f64) -> f64 {
    return (1.8*c + 32.0);
}

fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x*x);
}

fn say_a_number(number: i32) {
    println!("number is {}", number);
}

fn say_the_sum(a: i8, b: i8) {
    println!("sum is {}", a + b);
}

/* DATA TYPES */
fn data_types() {
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

/* BASICS */
fn basics() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b as f64 + c as f64)/(3 as f64);

    assert_eq!(average, 45.1);
    println!("Test passed!");

    /*******************/
    /* Char data types */
    /*******************/
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}", letter, number, finger);

    /*************************/
    /* Comparison opeartions */
    /*************************/
    let a = 1;
    let b = 2;
    println!("a is {} and b is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);

    /************************************/
    /* Boolean data type and operations */
    /************************************/
    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b {}", a ^ b);

    // a ^ b is true therefore rust will not process 
    // panic as this is always true no matter what is right
    let c = (a ^ b) || panic!();
    println!("c is {}", c);

    /* Bitwise Operations */
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}", value);

    value = !value; // NOT
    println!("value is {:08b}", value);

    value = value & 0b1111_0111; // clear bit with AND
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000); // check bit with AND

    value = value | 0b0100_0000; // set bit with OR
    println!("value is {:08b}", value);

    value = value ^ 0b0101_0101; // XOR
    println!("value is {:08b}", value);

    value = value << 4; // shift left by 4
    println!("value is {:08b}", value);

    value = value >> 2; // shift left by 2
    println!("value is {:08b}", value);

    /*************************/
    /* Arithmetic operations */
    /*************************/
    let a = 10;
    let b = 3;
    let c = a + b;
    println!("c is {}", c);

    let d = a - b;
    println!("d is {}", d);

    let e = a * b;
    println!("e is {}", e);

    let f = a / b;
    println!("f is {}", f);

    let g = a % b;
    println!("g is {}", g);

    let h = 10.0;
    let j = 3.0;
    let i = h / j;
    println!("i is {}", i);

    let k = a as f64 / j;
    println!("k is {:08.3}\na is {}", k, a);
    print!("k is {:08.3}\na is {}", k, a);
    println!("k is {0:08.3}\na is {1}\nonce again, k is {0}", k, a);

    let mut x: u8 = 255;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);

    let y: f32 = 10.012312321312312123213213;
    println!("y is {}", y);
}