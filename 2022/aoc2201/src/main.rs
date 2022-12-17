use aoc2201::*;

fn main() {
    let input = include_str!("../input.txt");

    let calories = most_calories(input);

    println!("Most calories: {calories}");
}
