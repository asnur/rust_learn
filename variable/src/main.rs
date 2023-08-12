fn main() {
    // Immutable variable
    let username = "Rustacean";
    println!("Hello, {}!", username);

    // Mutable variable
    let mut count = 0;
    println!("count = {}", count);

    count = 1;
    println!("count = {}", count);

    // Argument parameters macro
    println!("Hello, {name}!", name = "Rustacean");

    let number = 24;
    let message = "akwokowkow";

    println!("number = {1}, message = {0}", number, message);

    // Multiple Declaration
    let (x, y): (i32, i32) = (1, 2);

    println!("x = {}, y = {}", x, y);

    // Shadowing
    let x = 5;

    println!("x = {}", x);

    let x = x + 1;

    println!("x = {}", x);
}
