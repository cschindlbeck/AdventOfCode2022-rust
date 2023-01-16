use std::{collections::HashSet, fs};

fn main() {
    let filename: &str = "src/input2.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut cum_game: u64 = 0;
    for line in contents.lines() {
        let (line1, line2) = line.split_at(line.len() / 2);
        println!("Result : {}", line);
        println!("Result 1: {}", line1);
        println!("Result 2: {}", line2);
        let a = HashSet::from(line1);
        let b = HashSet::from(line2);
        // let num: Vec<u8> = line
        //     .chars()
        //     .filter(|c| !c.is_whitespace())
        //     .map(|c| c as u8)
        //     .map(|c| c - 65)
        //     .collect::<Vec<_>>();
        // let player1 = num[0];
        // let player2 = num[1] - 23;
        // let res: i32 = (player1 as i32 - player2 as i32).rem_euclid(3);
        // match res {
        //     1 => cum_game += 0, // println!("Player1 won"),
        //     2 => cum_game += 6, //println!("Player2 won"),
        //     0 => cum_game += 3, //println!("Draw"),
        //     _ => unreachable!(),
        // }
        // cum_game += 1 + player2 as u64;
    }
    println!("Result Pr 1: {}", cum_game);
}
