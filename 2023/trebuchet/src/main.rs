use std::fs;

struct Digit<'a> {
    name: &'a str,
    value: char,
}

const DIGITS: [Digit; 9] = [
    Digit { name : "one",  value:  '1'},
    Digit { name : "two",  value:  '2'},
    Digit { name : "three", value: '3'},
    Digit { name : "four", value:  '4'},
    Digit { name : "five", value:  '5'},
    Digit { name : "six",  value:  '6'},
    Digit { name : "seve", value:  '7'},
    Digit { name : "eigh", value:  '8'},
    Digit { name : "nine", value:  '9'},
];

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_first_number(mut str: String, reversed: bool) -> char {
    if reversed {
        str = str.chars().rev().collect::<String>();
    }

    for (index, character) in str.chars().enumerate() {
        if character.is_numeric(){
            return character;
        } else {
            for digit in DIGITS {
                
                let mut digit_str: String;
                digit_str = digit.name.to_string();
                
                if index + digit.name.len() > str.len(){
                    continue;
                }
                if reversed{
                    digit_str = digit_str.chars().rev().collect();    
                }
                
                if digit_str == &str[index..(index+digit.name.len())]{
                    return digit.value;
                }
            }
        }
    }
    return '0';
}

fn get_number(str: String) -> i32 {
    let first: char = get_first_number(str.clone(), false);
    let last: char = get_first_number(str.clone(), true);

    let n: i32 = format!("{}{}", first.to_string(), last).parse::<i32>().unwrap();

    println!("{}", n);

    return n;
}

fn main() {
    let file_path: &'static str = "demo_input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut sum: i32 = 0;

    for line in lines {
        sum += get_number(line.to_string());
    }

    println!("The result is : {}", sum);
}
