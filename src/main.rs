fn main() {

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
    // println!("{}\n{}\n{}", letter, number, finger);

    /*************************/
    /* Comparison opeartions */
    /*************************/
    let a = 1;
    let b = 2;
    // println!("a is {} and b is {}", a, b);
    // println!("a EQUAL TO b is {}", a == b);
    // println!("a NOT EQUAL TO b is {}", a != b);
    // println!("a GREATER THAN b is {}", a > b);
    // println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    // println!("a LESS THAN b is {}", a < b);
    // println!("a LESS THAN OR EQUAL TO b is {}", a <= b);

    /************************************/
    /* Boolean data type and operations */
    /************************************/
    let a = true;
    let b = false;
    // println!("a is {} and b is {}", a, b);
    // println!("NOT a is {}", !a);
    // println!("a AND b is {}", a & b);
    // println!("a OR b is {}", a | b);
    // println!("a XOR b {}", a ^ b);

    // a ^ b is true therefore rust will not process 
    // panic as this is always true no matter what is right
    let c = (a ^ b) || panic!();
    // println!("c is {}", c);

    /* Bitwise Operations */
    let mut value = 0b1111_0101u8;
    // println!("value is {}", value);
    // println!("value is {:08b}", value);

    value = !value; // NOT
    // println!("value is {:08b}", value);

    value = value & 0b1111_0111; // clear bit with AND
    // println!("value is {:08b}", value);
    // println!("bit 6 is {}", value & 0b0100_0000); // check bit with AND

    value = value | 0b0100_0000; // set bit with OR
    // println!("value is {:08b}", value);

    value = value ^ 0b0101_0101; // XOR
    // println!("value is {:08b}", value);

    value = value << 4; // shift left by 4
    // println!("value is {:08b}", value);

    value = value >> 2; // shift left by 2
    // println!("value is {:08b}", value);

    /*************************/
    /* Arithmetic operations */
    /*************************/
    let a = 10;
    let b = 3;
    let c = a + b;
    // println!("c is {}", c);

    let d = a - b;
    // println!("d is {}", d);

    let e = a * b;
    // println!("e is {}", e);

    let f = a / b;
    // println!("f is {}", f);

    let g = a % b;
    // println!("g is {}", g);

    let h = 10.0;
    let j = 3.0;
    let i = h / j;
    // println!("i is {}", i);

    let k = a as f64 / j;
    // println!("k is {:08.3}\na is {}", k, a);
    // print!("k is {:08.3}\na is {}", k, a);
    // println!("k is {0:08.3}\na is {1}\nonce again, k is {0}", k, a);

    let mut x: u8 = 255;
    // println!("x is {}", x);
    x = 20;
    // println!("x is {}", x);

    let y: f32 = 10.012312321312312123213213;
    // println!("y is {}", y);
}
