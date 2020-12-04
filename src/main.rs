use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
fn main() {
    // first_problem_first_part();
    // first_problem_second_part();
    // second_problem_first_part();
    // second_problem_second_part();
    // third_problem_first_part();
    // third_problem_second_part();
    fourth_problem_first_part();
    // fourth_problem_second_part();
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

fn third_problem_first_part() -> io::Result<()> {

    let file = File::open("./data/3.txt")?;
    let reader = BufReader::new(file);
    

    let mut array = vec![];


    for line in reader.lines() {
        let mut second_array = vec![];
        for c in line.expect("lines failed").chars() {
            
            second_array.push(c);
        }
        array.push(second_array);
    }   

    let mut second_index = 0;
    let mut number_of_trees = 0;
    let mut right_value = 3;

    for i in 0..array.len() {
        if second_index > array[i].len() -1 {
            let mut difference = second_index - array[i].len();
            second_index = difference;
        }
        if array[i][second_index] == '#'{
            number_of_trees+=1;
        }
        second_index +=right_value;
    }
    
    println!("{}", number_of_trees);
    Ok(())
}


fn third_problem_second_part() -> io::Result<()> {

    let file = File::open("./data/3.txt")?;
    let reader = BufReader::new(file);
    

    let mut location_graph = vec![];


    for line in reader.lines() {
        let mut location_line = vec![];

        for c in line.expect("lines failed").chars() {
            location_line.push(c);
        }
        location_graph.push(location_line);
    }   

    let mut right_value = 1;
    let mut down_value = 1;
    

    let mut paths = [[0usize; 2]; 5];
    paths[0][0] = 1;
    paths[0][1] = 1;

    paths[1][0] = 1;
    paths[1][1] = 3;

    paths[2][0] = 1;
    paths[2][1] = 5;

    paths[3][0] = 1;
    paths[3][1] = 7;

    paths[4][0] = 2;
    paths[4][1] = 1;


    let mut product_of_number_of_trees:u32 = 1;


    for path in paths.iter() {
        down_value = path[0];
        right_value = path[1];

        let mut right_index = 0;

        let mut down_index = 0;
        
        let mut number_of_trees = 0;

        println!("Path {}", down_value);

        while true {
            
            if (down_index >= location_graph.len()) {
                println!("Number of trees {}", number_of_trees);
                product_of_number_of_trees = product_of_number_of_trees * number_of_trees;
                break;
            }

            if right_index > location_graph[down_index].len() -1 {
                let mut difference = right_index - location_graph[down_index].len();
                right_index = difference;
            }

            if location_graph[down_index][right_index] == '#'{
                number_of_trees+=1;
            }
            // println!("{} {} {}", down_index, right_index, location_graph[down_index][right_index]);
            right_index += right_value;
            down_index += down_value;

        }
    }
        
    
    println!("{}", product_of_number_of_trees);
    Ok(())
}

fn fourth_problem_first_part() -> io::Result<()> {

    let file = File::open("./data/4.txt")?;
    let reader = BufReader::new(file);
    let mut passport_data:Vec<String> = vec![];
    let mut index = 0;
    let mut prev_line_string:String = "".to_string();

    for line in reader.lines() {
        let string:&str = &line?;

        if string.chars().count() == 0 {
            index +=1;
            prev_line_string = "".to_string();
        } else {

            if index == passport_data.len() {
                passport_data.push(string.to_string());
            } else {
                passport_data[index].push_str(string);
            }
        }
    }   
    
    let mut valid_passports = 0;

    for password_value in passport_data.iter() {
        if password_value.contains("byr") && password_value.contains("iyr") && password_value.contains("eyr") && password_value.contains("hgt") && password_value.contains("hcl") && password_value.contains("ecl") && password_value.contains("pid") {
            valid_passports +=1;
        }
    }

    println!("Valid passports {}", valid_passports);
    
    Ok(())
}