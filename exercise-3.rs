let my_num: i32 = 5;          // integer
let my_double: f64 = 5.99;    // float
let my_letter: char = 'D';    // character
let my_bool: bool = true;     // boolean
let my_text: &str = "Hello";  // string

let age: i32 = 25;
println!("Age is: {}", age);

let price: f64 = 19.99;
println!("Price is: ${}", price);

let myGrade: char = 'B';
println!("{}", myGrade);

let name: &str = "John";
println!("Hello, {}!", name);

let is_logged_in: bool = true;
println!("User logged in? {}", is_logged_in);

fn main() {
  let name = "John";
  let age = 28;
  let is_admin = false;
  
  println!("Name: {}", name);
  println!("Age: {}", age);
  println!("Admin: {}", is_admin);
}