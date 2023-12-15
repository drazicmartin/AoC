use std::fs;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Color<'a> {
    name: &'a str,
    max : i32,
}

impl Color<'_> {
    fn reset(&mut self) {
        self.max = 0;
    }
}

fn get_numbers(string : &str) -> i32 {
    let mut formed_string : String = String::new();
    for character in string.chars(){
        if character.is_numeric() {
            formed_string.push(character);
        }
    }
    
    formed_string.parse::<i32>().unwrap()
}


const COLORS: [Color; 3] = [
    Color { name : "blue",  max: 12 },
    Color { name : "red",   max: 13 }, 
    Color { name : "green", max: 14 },
];


fn is_game_valid(game: &str, rounds: &str) -> i32 {
    let mut stats: HashMap<&str, i32> = HashMap::new();
    for color in COLORS {
        stats.entry(color.name).or_insert(0);
    }

    for round in rounds.split(';'){
        for record in round.split(','){
            for color in COLORS {
                if record.contains(color.name){
                    let actual: i32 = get_numbers(record);
                    if actual > stats[color.name] {
                        stats.insert(color.name, actual);
                    }
                }
            }
        }
    }
    let mut result : i32 = 1;
    for color in COLORS {
        result *= stats[color.name];
    }

    result
}

fn main() {
    let file_path: &str = "input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut sum_game : i32 = 0;
    
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (game, rounds) : (&str, &str) = line.split_at(line.find(':').unwrap());
        sum_game += is_game_valid(game, rounds)
    }
    println!("Result : {}", sum_game);
}
