use std::fs;

fn main() {
    let filename: &str = "src/input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut max_calories: i64 = 0;
    let mut cum_calories: i64 = 0;
    for line in contents.lines() {
        if line.is_empty() {
            if cum_calories > max_calories {
                max_calories = cum_calories;
            }
            cum_calories = 0;
        }
        else {
            cum_calories += line.parse::<i64>().unwrap();
        }
    }

    println!("Result: {}", max_calories);
}
