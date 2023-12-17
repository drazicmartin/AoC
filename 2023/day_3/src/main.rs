use std::fs;
use std::thread;
use std::time::Instant;

fn sum_valid(top_line: &str, line: &str, bottom_line: &str) -> i32 {

    let zipper = line.chars()
        .zip(top_line.chars())
        .zip(bottom_line.chars())
        .map(|((x,y),z)| (y,x,z));
    
    let mut sum: i32 = 0;
    let mut is_building: bool = false;
    let mut last_symbol_distance: i32 = -1;
    let mut build: String = String::new();

    for (i, (top_char, middle_char, bottom_char)) in zipper.enumerate() {
        
        if is_symbol(top_char) || is_symbol(bottom_char) ||  is_symbol(middle_char){
            last_symbol_distance=0;
        }

        if middle_char.is_numeric(){
            is_building = true;
            build.push(middle_char);
        }
        
        if i == line.len()-1 || !middle_char.is_numeric(){
            if (!(last_symbol_distance < 0)) && is_building && build.len()+1 >= (last_symbol_distance as usize) {
                sum += build.parse::<i32>().unwrap();
            }
            build = String::new();
            is_building = false;
        }

        if last_symbol_distance >= 0{
            last_symbol_distance += 1;
        }
    }

    return sum;
}

fn is_symbol(char: char) -> bool {
    return (!char.is_numeric()) && char != '.';
}

fn main() {
    let file_path: &str = "input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut sum: i32 = 0;

    let start_time = Instant::now();

    let mut handles = Vec::new();

    for i in (0..lines.len()-1).collect::<Vec<usize>>() {
        let top_line: &str;
        let line: &str = lines[i];
        let bottom_line: &str;

        if i == 0 {
            top_line = lines[i];
        }else{
            top_line = lines[i-1];
        }

        if i == lines.len()-2 {
            bottom_line = lines[i];
        }else{
            bottom_line = lines[i+1];
        }

        let top_clone = top_line.to_owned(); // Cloning strings
        let line_clone = line.to_owned();
        let bottom_clone = bottom_line.to_owned();

        // let handle = thread::spawn(move || sum_valid(&top_clone, &line_clone, &bottom_clone));
        // handles.push(handle);

        sum += sum_valid(top_line, line, bottom_line);
    }

    // for handle in handles {
    //     sum += handle.join().unwrap();
    // }
    
    println!("Result : {}", sum);

    // Stop measuring time
    let end_time = Instant::now();

    // Calculate elapsed time
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
}
