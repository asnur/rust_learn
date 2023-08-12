fn main() {
    //== Primitive Scalar Types ==//
    let x: i32 = 5; // Integer
    let y: f64 = 5.0; // Floating-point
    let z: char = 'z'; // Character
    let t: bool = true; // Boolean
    let p: &i32 = &x; // Pointer

    println!("x = {}, y = {}, z = {}, t = {}, p = {}", x, y, z, t, p);

    //== String literal ==//
    let s: &str = "Hello, world!"; // String literal

    println!("s = {}", s);

    let multi_line_string: &str = "Hello,
    world!"; // Multi-line string

    println!("multi_line_string = {}", multi_line_string);

    let raw_string_1: &str = r"Hello, \n world!"; // Raw string

    println!("raw_string = {}", raw_string_1);

    let raw_string_2: &str = r#"Hello, "world!"#; // Raw string

    println!("raw_string = {}", raw_string_2);

    //== Constant ==//
    const MAX_POINTS: u32 = 100_000; // Constant
    const PI: f32 = 22.0 / 7.0; // Constant

    println!("MAX_POINTS = {}, PI = {}", MAX_POINTS, PI);
}
