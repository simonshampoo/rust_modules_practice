extern crate phrases; 

use phrases::english::{greetings, farewells};
use phrases::japanese;
fn main() {
	println!("Hello in English: {}", greetings::hello()); 
	println!("Hello in Japanese: {}", japanese::hello()); 
	println!("Goodbye in English: {}", farewells::goodbye()); 
	println!("Goodbye in Japanese: {}", japanese::goodbye()); 
} 
