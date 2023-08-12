fn main() {
    let x: i32 = 5;

    if x == 5 {
        println!("x = 5");
    } else {
        println!("x != 5");
    }

    let y: i32 = 5;

    if y == 5 {
        println!("y = 5");
    } else if y == 6 {
        println!("y = 6");
    } else {
        println!("y != 5 && y != 6");
    }

    let z: i32 = 5;

    let t: i32 = if z == 5 { 5 } else { 6 };

    println!("t = {}", t);

    let p: i32 = 5;

    let q: i32 = if p == 5 {
        println!("p = 5");
        5
    } else if p == 6 {
        println!("p = 6");
        6
    } else {
        println!("p != 5 && p != 6");
        0
    };

    println!("q = {}", q);
}
