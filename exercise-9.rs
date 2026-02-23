let numbers = [1, 2, 3, 4, 5];

let numbers = [1, 2, 3, 4, 5];
println!("The first number is: {}", numbers[0]);

let mut numbers = [1, 2, 3, 4, 5];
numbers[0] = 10;
println!("The new first number is: {}", numbers[0]);

let numbers = [1, 2, 3, 4, 5];
println!("This array has {} elements.", numbers.len());

let fruits = ["apple", "banana", "orange"];
for fruit in fruits {
  println!("I like {}.", fruit);
}

let numbers = [1, 2, 3, 4, 5];
println!("{:?}", numbers);

// An array with 3 elements
let mut cars = ["Volvo", "BMW", "Ford"];

// Trying to add another element (a fourth element) to the cars array will result in an error
cars[3] = "Mazda";   // Error: index out of bounds


// A vector with 3 elements
let mut cars = vec!["Volvo", "BMW", "Ford"];

// Add another element
cars.push("Mazda");

println!("{:?}", cars); // ["Volvo", "BMW", "Ford", "Mazda"]


let fruits = vec!["apple", "banana", "orange"];

let fruits = vec!["apple", "banana", "orange"];
println!("First fruit: {}", fruits[0]);

let mut fruits = vec!["apple", "banana", "orange"];
fruits[0] = "grape";
println!("New first fruit: {}", fruits[0]);

let mut fruits = vec!["apple", "banana"];
fruits.push("cherry");
println!("{:?}", fruits);

let mut fruits = vec!["apple", "banana", "cherry"];
fruits.pop();
println!("{:?}", fruits);

let mut fruits = vec!["banana", "orange"];
fruits.insert(0, "apple");
println!("{:?}", fruits);

let mut fruits = vec!["banana", "orange"];
fruits.insert(1, "apple");
println!("{:?}", fruits);

let mut fruits = vec!["apple", "banana", "orange"];
fruits.remove(0);
println!("{:?}", fruits);


let fruits = vec!["apple", "banana", "cherry"];
println!("There are {} fruits.", fruits.len());


let fruits = vec!["apple", "banana", "orange"];

for fruit in &fruits {
    println!("I like {}.", fruit);
}




let person = ("John", 30, true);
println!("Name: {}", person.0);
println!("Age: {}", person.1);
println!("Is active: {}", person.2);



let person = ("Jenny", 45, false);
let (name, age, active) = person;

println!("Name: {}", name);
println!("Age: {}", age);
println!("Active: {}", active);



fn get_user() -> (String, i32) {
  (String::from("Liam"), 25)
}

fn main() {
  let user = get_user();
  println!("User: {} ({} years old)", user.0, user.1);
}



use std::collections::HashMap;

fn main() {
  // Create a HashMap called capitalCities
  let mut capitalCities = HashMap::new();

  // Add keys and values (Country, City)
  capitalCities.insert("England", "London");
  capitalCities.insert("Germany", "Berlin");
  capitalCities.insert("Norway", "Oslo");

  println!("{:?}", capitalCities);
}

use std::collections::HashMap;

fn main() {
  let mut capitalCities = HashMap::new();

  capitalCities.insert("England", "London");
  capitalCities.insert("Germany", "Berlin");
  capitalCities.insert("Norway", "Oslo");
  
  if let Some(city) = capitalCities.get("England") {
    println!("The capital of England is {}.", city);
  } else {
    println!("England is not in the map.");
  }
}

use std::collections::HashMap;

fn main() {
  let mut capitalCities = HashMap::new();

  capitalCities.insert("England", "London");
  capitalCities.insert("England", "Berlin");

  println!("{:?}", capitalCities);
}


use std::collections::HashMap;

fn main() {
  let mut capitalCities = HashMap::new();

  // Add keys and values (Country, City)
  capitalCities.insert("England", "London");
  capitalCities.insert("Germany", "Berlin");
  capitalCities.insert("Norway", "Oslo");

  // Remove the key "England"
  capitalCities.remove("England");

  println!("{:?}", capitalCities);
}



use std::collections::HashMap;

fn main() {
  let mut capitalCities = HashMap::new();

  // Add keys and values (Country, City)
  capitalCities.insert("England", "London");
  capitalCities.insert("Germany", "Berlin");
  capitalCities.insert("Norway", "Oslo");

  // Loop through the HashMap
  for (country, city) in &capitalCities {
    println!("The capital of {} is {}.", country, city);
  }
}






// Create a Struct called Person
struct Person {
  name: String,
  age: u32,
  can_vote: bool,
}

// Create a Person object
let user = Person {
  name: String::from("John"),
  age: 35,
  can_vote: true,
};

// Access and print the values
println!("Name: {}", user.name);
println!("Age: {}", user.age);
println!("Can vote? {}", user.can_vote);

fn main() {
  struct Person {
    name: String,
    age: u32,
  }
  
  let mut user = Person {
    name: String::from("John"),
    age: 35,
  };

  user.age = 36;  // Change value of age
  println!("Name: {}", user.name);
  println!("Updated age: {}", user.age);
}



enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let my_direction = Direction::Up;
    println!("We are going up!");
}


enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let my_direction = Direction::Left;

    match my_direction {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Going right"),
    }
}


enum LoginStatus {
    Success(String),
    Error(String),
}

fn main() {
    let result1 = LoginStatus::Success(String::from("Welcome, John!"));
    let result2 = LoginStatus::Error(String::from("Incorrect password"));

    match result1 {
        LoginStatus::Success(message) => println!("Success: {}", message),
        LoginStatus::Error(message) => println!("Error: {}", message),
    }
}