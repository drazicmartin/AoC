use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn get_winning_and_played(line: &str) -> (Vec<i32>, Vec<i32>) {
    let mut winning: Vec<i32> = Vec::new();
    let mut played: Vec<i32> = Vec::new();

    let mut build: String = String::new();
    let mut is_winning_set = false;
    let mut is_played_set = false;

    for (idx, character) in line.chars().enumerate() {
        if character == ':' {
            is_winning_set = true;
        } else if character == '|' {
            is_winning_set = false;
            is_played_set = true;
        }

        if is_played_set || is_winning_set {
            if character.is_numeric() {
                build.push(character);
            } 
            
            if (idx == line.len()-1 || character == ' ') && !build.is_empty() {
                if is_played_set {
                    played.push(build.parse::<i32>().unwrap());
                } else if is_winning_set {
                    winning.push(build.parse::<i32>().unwrap());
                }
                build = String::new();
            }
        }
    }

    return (winning, played);
}

fn get_score_part1(winning: Vec<i32>, played: Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    for play in played {
        if winning.contains(&play) {
            if result == 0 {
                result = 1;
            } else {
                result *= 2;
            }
        }
    }

    result
}

fn get_score_part2(winning: Vec<i32>, played: Vec<i32>) -> i32 {
    let mut n : i32 = 0;

    for play in played {
        if winning.contains(&play) {
            n+=1;
        }
    }

    n
}

fn update_hasmap(n: i32, card_number: i32, mut card_multiplier: HashMap<i32, i32>) -> HashMap<i32, i32>{

    for i in 1..=(n) {
        *card_multiplier.entry(i + card_number).or_insert(0) += 1;
    }

    card_multiplier
}

fn main() {
    let file_path: &str = "demo_input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut sum: i32 = 0;

    let start_time = Instant::now();

    let mut card_multiplier: HashMap<i32, i32> = HashMap::new();
    let mut card_number: i32 = 0;

    for i in (0..lines.len()-1).collect::<Vec<usize>>() {
        let line = lines[i];

        card_number = (i+1) as i32;

        let (winning, played) = get_winning_and_played(line);

        // println!("{:?}, {:?}", winning, played);
        // println!("{}, {}", i, get_score(winning.clone(), played.clone()));

        let n: i32 = get_score_part2(winning, played);

        *card_multiplier.entry(card_number).or_insert(0) += 1;

        card_multiplier = update_hasmap(n, card_number, card_multiplier);

        let mut mult = if (i==0){
            1
        }else{
            *card_multiplier.get(&(card_number-1)).unwrap()
        };
        sum += card_multiplier.get(&card_number).unwrap() * mult;
    }

    println!("{:?}", card_multiplier);

    println!("Result : {}", sum);

    // Stop measuring time
    let end_time = Instant::now();

    // Calculate elapsed time
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
}
