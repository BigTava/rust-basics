fn main() {
    let mut x: u8 = 255;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);

    let y: f32 = 10.012312321312312123213213;
    println!("y is {}", y);

    /* Arithmetic operations */
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
    println!("k is {}", k);
}
