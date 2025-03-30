use num::complex::Complex;
fn main() {
    println!("Hello, world!");
    // test_Overflow();
    test_Range();
    test_Complex();
}


fn test_Overflow(){
    let mut x: u8 = 255;
    x = x + 1;
    println!("{}", x);
}

fn test_Range(){
    for i in 1..=5{
        println!("{}", i);
    }
}

fn test_Complex(){
    let mut c = Complex::new(3.0, 4.0);
    println!("c = {}", c);
    println!("c.re = {}", c.re);
    println!("c.im = {}", c.im);
    c = c * Complex::new(2.0, 3.0);
    println!("c = {}", c);
}