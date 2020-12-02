use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
fn main() {
    // first_problem_first_part();
    // first_problem_second_part();
    // second_problem_first_part();
    // second_problem_second_part();
}


fn first_problem_first_part() -> io::Result<()> {

    let mut file = File::open("./data/1.txt")?;
    let reader = BufReader::new(file);
    let mut first_number: Option<u32> = None;
    let mut second_number: Option<u32> = None;

    let mut numbers = Vec::new();
    let mut sum: u32 = 0;
    let mut product: u32 = 1;
    
    
    for line in reader.lines() {
        let mut sum = 0;
        let i = line?;
        let number: u32 = i.parse::<u32>().unwrap();
        numbers.push(number);
    }

    let numbers_length = numbers.len();
    for i in 0..numbers_length {

        for j in i+1..numbers_length {
            sum = numbers[i] as u32 + numbers[j] as u32;
            if sum == 2020 {
                println!("Sum is {} {} {}",numbers[i], numbers[j], sum);
                product = numbers[i] as u32 * numbers[j] as u32;
                break;
            }
        }
    }
    println!("Question 1a {}", product);
    Ok(())
}


fn first_problem_second_part() -> io::Result<()> {

    let mut file = File::open("./data/1.txt")?;
    let reader = BufReader::new(file);
    let mut first_number: Option<u32> = None;
    let mut second_number: Option<u32> = None;

    let mut numbers = Vec::new();
    let mut sum: u32 = 0;
    let mut product: u32 = 1;
    
    
    for line in reader.lines() {
        let mut sum = 0;
        let i = line?;
        let number: u32 = i.parse::<u32>().unwrap();
        numbers.push(number);
    }

    let numbers_length = numbers.len();
    for i in 0..numbers_length {

        for j in i+1..numbers_length {
            for k in j+1..numbers_length {
            sum = numbers[i] as u32 + numbers[j] as u32 + numbers[k] as u32;
            if sum == 2020 {
                println!("Sum is {} {} {}, {}",numbers[i], numbers[j], numbers[k], sum);
                product = numbers[i] as u32 * numbers[j] as u32 *  numbers[k] as u32;
                break;
            }
        }
    }
    }
    println!("Question 1a {}", product);
    Ok(())
}


fn second_problem_first_part() -> io::Result<()> {

    let file = File::open("./data/2.txt")?;
    let reader = BufReader::new(file);
    let mut lower_limit: u32 = 0;
    let mut upper_limit: u32 = 0;
    let mut total_valid_passwords: u32 = 0;

    let mut eligible_character = "";
    let mut password_string = "";
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    
    for line in reader.lines() {
        let string = &line.unwrap();

        for cap in re.captures_iter(string) {
            lower_limit = cap[1].parse::<u32>().unwrap();
            upper_limit = cap[2].parse::<u32>().unwrap();

            eligible_character = &cap[3];
            password_string = &cap[4];

            let c = password_string.matches(eligible_character).count();
            if c as u32 >= lower_limit && c as u32 <= upper_limit {
                total_valid_passwords+=1;
            }
            
        }
    }
    println!("{} ", total_valid_passwords);
    Ok(())
}

fn second_problem_second_part() -> io::Result<()> {

    let file = File::open("./data/2.txt")?;
    let reader = BufReader::new(file);
    let mut lower_index: u32 = 0;
    let mut upper_index: u32 = 0;
    let mut total_valid_passwords: u32 = 0;

    let mut eligible_character = "";
    let mut password_string = "";
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    
    for line in reader.lines() {
        let mut is_valid = false;
        let string = &line.unwrap();
        
        for cap in re.captures_iter(string) {
            lower_index = cap[1].parse::<u32>().unwrap();
            upper_index = cap[2].parse::<u32>().unwrap();

            eligible_character = &cap[3];
            password_string = &cap[4];

            let password_string_vector: Vec<char> = password_string.chars().collect();

            if (password_string_vector.get(lower_index as usize -1) == Some(&eligible_character.chars().nth(0).unwrap())) {
                is_valid = !is_valid;
            }
            

            if (password_string_vector.get(upper_index as usize -1) == Some(&eligible_character.chars().nth(0).unwrap())) {
                is_valid = !is_valid;
            }


            if is_valid {
                total_valid_passwords += 1;
            }
            
        }
    }
    println!("{} ", total_valid_passwords);
    Ok(())
}
