#![no_std]
#![no_main]

#[macro_use]
extern crate user;

#[macro_use]
extern crate alloc;

#[no_mangle]
pub fn main() -> usize {
    for _ in 0..10 {
        panic!("Hello world! from user mode program!");
    }
    0
}
