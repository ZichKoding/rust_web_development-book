fn main() {
    println!("Hello, world!");

    // Rust's primitive data types:
    // Integer types: i8, i16, i32, i64, u8, u16, u32, u64, isize, usize
    // Floating-point types: f32, f64
    // Boolean type: bool
    // Character type: char

    // Variables
    let x: i32;
    let y: i32;
    let z: i32;
    let a: bool;
    let b: char;
    let c: f64;

    // Assignments
    x = 5;
    y = 10;
    println!("x: {}, y: {}", x, y);

    z = x + y;
    println!("z: {}", z);
    
    a = true;
    b = 'A';
    c = 3.14;
    println!("a: {}, b: {}, c: {}", a, b, c);
}
