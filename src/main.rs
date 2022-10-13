extern crate phrases; 

fn main() {
	println!("Hello in English: {}", phrases::english::greetings::hello()); 
	println!("Hello in Japanese: {}", phrases::english::greetings::hello()); 

	println!("Goodbye in English: {}", phrases::english::farewells::goodbye()); 
	println!("Goodbye in Japanese: {}", phrases::japanese::farewells::goodbye());
} 
