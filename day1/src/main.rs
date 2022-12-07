// --- Part Two ---
// By the time you calculate the answer to the Elves' question,
// they've already realized that the Elf carrying the most Calories
// of food might eventually run out of snacks.

// To avoid this unacceptable situation, the Elves would instead like
// to know the total Calories carried by the top three Elves
// carrying the most Calories. That way, even if one of those
// Elves runs out of snacks, they still have two backups.

// In the example above, the top three Elves are the
// fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories),
// then the fifth Elf (with 10000 Calories).

// The sum of the Calories carried by these three elves is 45000.

// Find the top three Elves carrying the most Calories.
// How many Calories are those Elves carrying in total?

use std::io;

fn main() {
  let mut current_calories: u32 = 0;
  let mut top_elf: u32 = 0;
  let mut second_elf: u32 = 0;
  let mut third_elf: u32 = 0;

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
      if current_calories > top_elf {
        third_elf = second_elf;
        second_elf = top_elf;
        top_elf = current_calories;
        println!("New Record! [{}]", top_elf);
      } else if current_calories > second_elf {
        third_elf = second_elf;
        second_elf = current_calories;
        println!("New Second Place! [{}]", second_elf);
      } else if current_calories > third_elf {
        third_elf = current_calories;
        println!("New Third Place! [{}]", third_elf);
      }
      current_calories = 0;
    };
  };
  println!("Top 3 elves: [{} {} {}]", top_elf, second_elf, third_elf);
  let total_top3: u32 = top_elf + second_elf + third_elf;
  println!("Total Calorie Count: [{}]", total_top3);
}
