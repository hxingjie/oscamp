#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
//use axstd::println_with_color;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("[WithColor]: Hello, Arceos!");
    //println_with_color("[WithColor]: Hello, Arceos!", "blue");
}
