use std::fs;

pub fn day_1() {
    let path = "src/day_1_input.txt";
    let mut calories_vec = Vec::new();
    let mut current_calories = 0;
    for line in fs::read_to_string(path).unwrap().lines() {
        if line.is_empty() {
            calories_vec.push(current_calories);
            current_calories = 0;
            continue;
        }
        let num = str::parse::<i32>(line).expect("Error parsing number");
        current_calories += num;
    }

    calories_vec.sort_unstable();

    let mut sum = 0;
    for _ in 0..3 {
        sum += calories_vec.pop().expect("Expected to pop");
    }

    println!("Sum: {}", sum);
}
