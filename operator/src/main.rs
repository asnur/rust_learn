fn main() {
    // Arithmetic
    let x: i32 = 5 + 5;
    let y: i32 = 5 - 5;
    let z: i32 = 5 * 5;
    let t: i32 = 5 / 5;

    println!("x = {}, y = {}, z = {}, t = {}", x, y, z, t);

    // Comparison and Logical and Negation
    let x: bool = 5 == 5;
    let y: bool = 5 != 5;
    let z: bool = 5 > 5;
    let t: bool = 5 < 5;
    let p: bool = 5 >= 5;
    let q: bool = 5 <= 5;
    let r: bool = true && false;
    let s: bool = true || false;
    let u: bool = !true;
    let v: bool = !false;
    let w: bool = 5 % 5 == 0;

    println!(
        "x = {}, y = {}, z = {}, t = {}, p = {}, q = {}, r = {}, s = {}, u = {}, v = {}, w = {}",
        x, y, z, t, p, q, r, s, u, v, w
    ); // x = true, y = false, z = false, t = false, p = true, q = true, r = false, s = true, u = false, v = true, w = true

    print!(
        "x = {x}, y = {y}, z = {z}, t = {t}",
        x = x,
        y = y,
        z = z,
        t = t
    ); // x = true, y = false, z = false, t = false

    // Bitwise
    let x: i32 = 5 & 5;
    let y: i32 = 5 | 5;
    let z: i32 = 5 ^ 5;
    let t: i32 = !5;
    let p: i32 = 5 << 5;
    let q: i32 = 5 >> 5;

    println!(
        "x = {}, y = {}, z = {}, t = {}, p = {}, q = {}",
        x, y, z, t, p, q
    );
}
