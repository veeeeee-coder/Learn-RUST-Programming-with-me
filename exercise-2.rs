let name="veee";
println!("My first name is: {}", name);
/*What is {}?
Rust uses {} as a placeholder in println!() to show variable values.*/

let name="vee";
let age=13;
println!("{} is {} years old",name,age);
/*The first {} gets replaced with name ("John")
The second {} gets replaced with age (30)*/


fn main() {
  let x = 5;
  x = 10; // Error
  println!("{}", x);
}
/*Variable Values Cannot be Changed by Default
By default, variables in Rust cannot be changed after they are created*/

let mut x = 5;
println!("Before: {}", x);
x = 10;
println!("After: {}", x);

/*Change Variable Values
If you want to change the value of a variable, you must use the mut keyword (which means mutable/changeable) */


