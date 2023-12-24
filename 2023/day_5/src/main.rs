use itertools::Itertools;
use std::fs;
use std::time::Instant;


struct RangeValue {
    start: u64,
    length: u64,
}

impl RangeValue {
    fn to_values(&self) -> Vec<u64> {
        (self.start..self.start + self.length).collect::<Vec<u64>>()
    }
}

fn get_numbers(line: &str) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();

    let mut build: String = String::new();

    for (idx, character) in line.chars().enumerate() {
        if character.is_numeric() {
            build.push(character);
        }

        if (idx == line.len() - 1 || character == ' ') && !build.is_empty() {
            match build.parse::<i64>() {
                Ok(x) => numbers.push(x),
                _ => println!("{}", build),
            }
            build = String::new();
        }
    }

    numbers
}

fn get_seed_part_2(values: Vec<i64>) -> Vec<i64> {
    let mut seeds: Vec<i64>= Vec::new();

    for i in (0..values.len()).step_by(2) {
        let init = values[i];
        let range = values[i + 1];
        for i in init..init+range{
            seeds.push(i);
        }
    }
    seeds
}

fn update_values(
    values: Vec<i64>,
    dest_start: i64,
    src_start: i64,
    length: i64,
) -> (Vec<i64>, Vec<i64>) {
    let mut updated_values: Vec<i64> = Vec::new();
    let mut not_updated_values: Vec<i64> = Vec::new();
    for value in values {
        if src_start <= value && value <= src_start + length {
            updated_values.push(dest_start + value - src_start)
        } else {
            not_updated_values.push(value);
        }
    }
    (updated_values, not_updated_values)
}

fn main() {
    let file_path: &str = "input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let start_time = Instant::now();

    let binding = get_seed_part_2(get_numbers(lines[0]));
    let chunk_size: i64 = 100000000;
    let all_values = binding.chunks(chunk_size.try_into().unwrap());

    // Stop measuring time
    let end_time = Instant::now();

    // Calculate elapsed time
    let elapsed_time = end_time - start_time;
    let n: i64 = all_values.len().try_into().unwrap();

    println!("Starting time: {:?} for creating {} seeds", elapsed_time, n*chunk_size);

    let mut updated_values: Vec<i64> = Vec::new();
    let mut location: Vec<i64> = Vec::new();

    for (idx, chunk) in all_values.enumerate() {
        let mut values: Vec<i64> = chunk.to_vec();
        for i in (1..=lines.len() - 1).collect::<Vec<usize>>() {
            let line = lines[i];
    
            if !line.is_empty() {
                let (dest_start, src_start, length): (i64, i64, i64) = match get_numbers(line)
                    .into_iter()
                    .collect_tuple::<(i64, i64, i64)>()
                {
                    Some(x) => x,
                    _ => (0, 0, 0),
                };
                let mut new_updated_values: Vec<i64>;
                (new_updated_values, values) = update_values(values, dest_start, src_start, length);
                updated_values.append(&mut new_updated_values);
            }
    
            if line.contains("map") || i == lines.len() - 1 {
                values.append(&mut updated_values);
                updated_values = Vec::new();
            }
        }
        location.append(&mut values);
        println!("Done {} %", ((idx as i64)/n)*100);
        println!("Done {}/{}", (idx as i64), n);
    }


    println!("Result : {:?}", location.iter().min());

    // Stop measuring time
    let end_time = Instant::now();

    // Calculate elapsed time
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
}
