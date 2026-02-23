fn function_name() {
  // code to be executed
}

// Create a function
fn say_hello() {
  println!("Hello from a function!");
}

say_hello(); // Call the function

fn greet(name: &str) {
  println!("Hello, {}!", name);
}

greet("John");

fn add(a: i32, b: i32) -> i32 {
  return a + b;
}

let sum = add(3, 4);
println!("Sum is: {}", sum);

fn add(a: i32, b: i32) -> i32 {
  a + b
}

let sum = add(3, 4);
println!("Sum is: {}", sum);


fn myFunction() {
  let message = "Hello!";
  println!("{}", message);  // You can access the message variable here
}

myFunction();

println!("{}", message); // Error - you cannot access the message variable outside of the function


let score = 80;

if score > 50 {
  let result = "Pass";
  println!("Result: {}", result);
}

println!("Result: {}", result); // Error: result is out of scope here

let x = 5;
let x = 10;

println!("x is: {}", x); // prints 10

