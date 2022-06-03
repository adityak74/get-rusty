use sys_info::{mem_info};
use std::f64;

fn main() {
    let result = mem_info().unwrap();
    println!("Total: {}", result.total);
    println!("Avail: {}", result.avail);
    println!("Free: {}", result.free);
}
