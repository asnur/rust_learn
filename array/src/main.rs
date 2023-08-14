use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("arr[0] = {}", arr[0]);

    // Array with loop
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
    }

    // Array with loop and sleep
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);

        sleep(Duration::from_secs(1));
    }
}
