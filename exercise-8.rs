let greeting: &str = "Hello";
println!("{}", greeting);


let text1 = "Hello World".to_string();

let text2 = String::from("Hello World");

let mut greeting = String::from("Hello");
greeting.push_str(" World");
println!("{}", greeting); // Hello World

let mut word = String::from("Hi");
word.push('!');
println!("{}", word); // Hi!


let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = format!("{} {} {}", s1, s2, s3);
println!("{}", result);


let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = s1 + " " + &s2 + " " + &s3;
println!("{}", result);

let name = String::from("John");
println!("Length: {}", name.len()); // 4

