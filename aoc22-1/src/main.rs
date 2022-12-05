use std::fs;

fn main() {
    let filename: &str = "src/input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut max_calories: i64 = 0;
    let mut cum_calories: i64 = 0;
    let mut vec: Vec<i64> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            vec.push(cum_calories);
            if cum_calories > max_calories {
                max_calories = cum_calories;
            }
            cum_calories = 0;
        } else {
            cum_calories += line.parse::<i64>().unwrap();
        }
    }
    vec.sort();
    vec.reverse();
    println!("Result Pr 1: {}", max_calories);
    println!("Result Pt 2: {}", vec[0] + vec[1] + vec[2]);
}
