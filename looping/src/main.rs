use std::thread::sleep;
use std::time::Duration;

fn main() {
    // While loop
    let mut x: i32 = 0;

    while x <= 10 {
        println!("x = {}", x);

        x += 1;
    }

    // While loop with sleep
    let mut y: i32 = 0;

    while y <= 10 {
        println!("y = {}", y);

        y += 1;

        sleep(Duration::from_secs(1));
    }

    // For loop
    for z in 0..10 {
        println!("z = {}", z);
    }

    // Keywork loop with break and continue
    let mut z = 0;

    loop {
        println!("z = {}", z);

        z += 1;

        if z == 5 {
            continue;
        }

        if z == 10 {
            break;
        }
    }

    // Loop with label
    let mut i = 0;
    let max = 9;

    'outer: loop {
        i += 1;
        let mut j = 0;

        'inner: loop {
            if i > max {
                break 'outer;
            }

            j += 1;
            if j > i {
                break 'inner;
            }

            print!("{i} ");
        }

        println!();
    }

    // Return value from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {}", result);

    // For loop with range
    for i in 0..10 {
        println!("i = {}", i);
    }

    // For loop with range and step
    for i in (0..10).step_by(2) {
        println!("i = {}", i);
    }

    // For loop with range and reverse
    for i in (0..10).rev() {
        println!("i = {}", i);
    }

    // For loop with range and reverse and step
    for i in (0..10).rev().step_by(2) {
        println!("i = {}", i);
    }
}
