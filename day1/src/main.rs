// The jungle must be too overgrown and difficult to navigate in vehicles or access from the air;
// the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin
// taking inventory of their supplies. One important consideration is food - in particular, the 
// number of Calories each Elf is carrying (your puzzle input).

// The Elves take turns writing down the number of Calories contained by the various meals, snacks,
// rations, etc. that they've brought with them, one item per line. Each Elf separates their own
// inventory from the previous Elf's inventory (if any) by a blank line.

// For example, suppose the Elves finish writing their items' Calories and end up with the following list:

// 1000
// 2000
// 3000

// 4000

// 5000
// 6000

// 7000
// 8000
// 9000

// 10000

// This list represents the Calories of the food carried by five Elves:

//     The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
//     The second Elf is carrying one food item with 4000 Calories.
//     The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
//     The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
//     The fifth Elf is carrying one food item with 10000 Calories.

// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask:
// they'd like to know how many Calories are being carried by the Elf carrying the most Calories.
// In the example above, this is 24000 (carried by the fourth Elf).

// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

use std::io;

fn main() {
  let mut current_calories: u32 = 0;
  let mut max_calories: u32 = 0;

  let lines = io::stdin().lines();

  for line in lines {
    let line_val: u32 = match line.unwrap().parse() {
      Ok(line) => line,
      Err(..) => 0,
    };

    if line_val > 0 {
      current_calories = current_calories + line_val;
      println!("Read Value: [{}] Current Running Total: [{}]", line_val, current_calories);
    } else {
      // If line is just whitespace or EOF, we're done with this elf:
      // If current calorie count > biggest total so far, update biggest so far
      println!("Done Reading, Total: [{}]", current_calories);
      if current_calories > max_calories {
        max_calories = current_calories;
        println!("New Record! [{}]", max_calories);
      }
      current_calories = 0
    };
  };
  println!("Largest Calorie Count: [{}]", max_calories);
}