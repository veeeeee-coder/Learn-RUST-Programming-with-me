let mut count = 1;

while count <= 5 {
  println!("Count: {}", count);
  count += 1;
}
/*In the example above, the loop keeps running as long as the counter is less than or equal to 5.
It prints the numbers from 1 to 5, one on each line.*/

let count = 10;

while count <= 5 {
  println!("This won't be printed.");
}



let mut num = 1;

while num <= 10 {
  if num == 6 {
    break;
  }
  println!("Number: {}", num);
  num += 1;
}

//This prints numbers from 1 to 5 (stops the loop when num reaches 6).


let mut num = 1;

while num <= 10 {
  if num == 6 {
    num += 1;
    continue;
  }

  println!("Number: {}", num);
  num += 1;
}
//This prints numbers from 1 to 10, except for the number 6.


for i in 1..6 {
  println!("i is: {}", i);
}
/*This prints numbers from 1 to 5.

Note: 1..6 means from 1 up to (but not including) 6.

Note: Rust handles the counter variable (i) automatically, unlike many other programming languages. You don't need to declare or increment it manually.*/


for i in 1..=6 {
  println!("i is: {}", i);
}
//This prints numbers from 1 to 6, including 6.

for i in 1..=10 {
  if i == 3 {
    continue; // skip 3
  }
  if i == 5 {
    break; // stop before printing 5
  }
  println!("i is: {}", i);
}

//This prints 1, 2, and 4. It skips 3 and stops before 5.