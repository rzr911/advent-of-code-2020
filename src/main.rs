use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// use regex::Regex;
use fancy_regex::Regex;
fn main() {
    // first_problem_first_part();
    // first_problem_second_part();
    // second_problem_first_part();
    // second_problem_second_part();
    // third_problem_first_part();
    // third_problem_second_part();
    // fourth_problem_first_part();
    // fourth_problem_second_part();

    // fifth_problem_first_part();
    fifth_problem_second_part();
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


// fn second_problem_first_part() -> io::Result<()> {

//     let file = File::open("./data/2.txt")?;
//     let reader = BufReader::new(file);
//     let mut lower_limit: u32 = 0;
//     let mut upper_limit: u32 = 0;
//     let mut total_valid_passwords: u32 = 0;

//     let mut eligible_character = "";
//     let mut password_string = "";
//     let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    
//     for line in reader.lines() {
//         let string = &line.unwrap();

//         for cap in re.captures(string) {
//             lower_limit = cap[1].parse::<u32>().unwrap();
//             upper_limit = cap[2].parse::<u32>().unwrap();

//             eligible_character = &cap[3];
//             password_string = &cap[4];

//             let c = password_string.matches(eligible_character).count();
//             if c as u32 >= lower_limit && c as u32 <= upper_limit {
//                 total_valid_passwords+=1;
//             }
            
//         }
//     }
//     println!("{} ", total_valid_passwords);
//     Ok(())
// }

// fn second_problem_second_part() -> io::Result<()> {

//     let file = File::open("./data/2.txt")?;
//     let reader = BufReader::new(file);
//     let mut lower_index: u32 = 0;
//     let mut upper_index: u32 = 0;
//     let mut total_valid_passwords: u32 = 0;

//     let mut eligible_character = "";
//     let mut password_string = "";
//     let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    
//     for line in reader.lines() {
//         let mut is_valid = false;
//         let string = &line.unwrap();
        
//         for cap in re.captures_iter(string) {
//             lower_index = cap[1].parse::<u32>().unwrap();
//             upper_index = cap[2].parse::<u32>().unwrap();

//             eligible_character = &cap[3];
//             password_string = &cap[4];

//             let password_string_vector: Vec<char> = password_string.chars().collect();

//             if (password_string_vector.get(lower_index as usize -1) == Some(&eligible_character.chars().nth(0).unwrap())) {
//                 is_valid = !is_valid;
//             }
            

//             if (password_string_vector.get(upper_index as usize -1) == Some(&eligible_character.chars().nth(0).unwrap())) {
//                 is_valid = !is_valid;
//             }


//             if is_valid {
//                 total_valid_passwords += 1;
//             }
            
//         }
//     }
//     println!("{} ", total_valid_passwords);
//     Ok(())
// }

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

fn fourth_problem_second_part() -> io::Result<()> {

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
                passport_data[index].push_str(" ");
                passport_data[index].push_str(string);
            }
        }
    }   
    
    let mut valid_passports = 0;

    let re = Regex::new(r"(?=.*(byr):([0-9]{4}))(?=.*(iyr):([0-9]{4}))(?=.*(eyr):([0-9]{4}))(?=.*(hgt):([0-9]{2,3})(cm|in))(?=.*(ecl):([a-z]{3}))(?=.*(pid):([0-9]+))(?=.*(hcl):(#[0-9a-f]{6}))").unwrap();
    let valid_ecl_values = vec!["amb","blu", "brn", "gry", "grn", "hzl", "oth"];

    for passport_value in passport_data.iter() {
        let mut is_valid = true;

        let captures =  re.captures(passport_value);
        let wrapped_captures = &captures.unwrap();
        
        if (wrapped_captures.is_some()) {
            let byr = wrapped_captures.as_ref().unwrap().get(2).unwrap().as_str().parse::<i32>().unwrap();
            let iyr = wrapped_captures.as_ref().unwrap().get(4).unwrap().as_str().parse::<i32>().unwrap();
            let eyr = wrapped_captures.as_ref().unwrap().get(6).unwrap().as_str().parse::<i32>().unwrap();

            let hgt_value = wrapped_captures.as_ref().unwrap().get(8).unwrap().as_str().parse::<i32>().unwrap();
            let hgt_unit = wrapped_captures.as_ref().unwrap().get(9).unwrap().as_str();
            let ecl = wrapped_captures.as_ref().unwrap().get(11).unwrap().as_str();
            
            let pid = wrapped_captures.as_ref().unwrap().get(13).unwrap().as_str();

            if pid.chars().count() != 9 {
                println!("{}", pid.chars().count());
                is_valid = false;
            }

            let hcl = wrapped_captures.as_ref().unwrap().get(15).unwrap().as_str();

            if !(fourth_problem_is_height_valid(hgt_value, hgt_unit)){
                is_valid = false;
            }

            if !(if_string_in_vector(ecl, &valid_ecl_values)) {
                is_valid = false;
            }

            if byr < 1920 || byr > 2002 || iyr < 2010 || iyr > 2020 || eyr < 2020 || eyr > 2030 {
                is_valid = false;
            }

            if is_valid {
                println!("{},{},{},{}{},{},{},{}",byr,iyr,eyr,hgt_value,hgt_unit, ecl,pid, hcl );
                valid_passports +=1;
            }
            
            
        }

    }

    println!("Valid passports {}", valid_passports);
    
    Ok(())
}

fn fourth_problem_is_height_valid(value:i32, unit: &str) -> bool {
    
    if unit == "cm" && value >= 150 && value  <= 193 {
        return true; 
    }

    if unit == "in" && value >= 59 && value <= 76 {
        return true;
    }

    return false;
}

fn if_string_in_vector(value: &str, vector: &Vec<&str>) -> bool {

    if vector.iter().any(|&i| i==value) {
        return true;
    }

    return false;
}


fn fifth_problem_first_part()-> io::Result<()> {
    let file = File::open("./data/5.txt")?;
    let reader = BufReader::new(file);
    let mut passport_data:Vec<String> = vec![];
    let mut index = 0;

    let number: u32 = 2;
    let row_length:u32 = 7;
    let mut range : u32 = number.pow((row_length-index)) - 1;
    let mut row_index_array:[u32;2] = [0, range];
    let mut column_index_array:[u32;2] = [0, row_length];
    let mut highest_seat_id: u32 = 0;

    for line in reader.lines() {
        row_index_array = [0, range];
        column_index_array = [0, row_length];
        let passport:&str = &line?;
        let passport_chars: Vec<char> = passport.chars().collect();

        for i in 0..row_length {
            row_index_array = fifth_problem_get_row(passport_chars[i as usize], row_index_array);
        }
        
        for i in row_length..10 {
            column_index_array = fifth_problem_get_row(passport_chars[i as usize], column_index_array);
        }

        println!("{}", row_index_array[0]);
        println!("{}", column_index_array[0]);

        let seat_id: u32 = row_index_array[0] * 8 + column_index_array[0];

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }
    println!("{}", highest_seat_id);
    Ok(())
}

fn fifth_problem_second_part()-> io::Result<()> {
    let file = File::open("./data/5.txt")?;
    let reader = BufReader::new(file);
    let mut passport_data:Vec<String> = vec![];
    let mut index = 0;

    let number: u32 = 2;
    let row_length:u32 = 7;
    let mut range : u32 = number.pow((row_length-index)) - 1;
    let mut row_index_array:[u32;2] = [0, range];
    let mut column_index_array:[u32;2] = [0, row_length];
    let mut highest_seat_id: u32 = 0;
    
    let mut seat_id_vector = vec![];

    for line in reader.lines() {
        row_index_array = [0, range];
        column_index_array = [0, row_length];
        let passport:&str = &line?;
        let passport_chars: Vec<char> = passport.chars().collect();

        for i in 0..row_length {
            row_index_array = fifth_problem_get_row(passport_chars[i as usize], row_index_array);
        }
        
        for i in row_length..10 {
            column_index_array = fifth_problem_get_row(passport_chars[i as usize], column_index_array);
        }


        let seat_id: u32 = row_index_array[0] * 8 + column_index_array[0];
        seat_id_vector.push(seat_id);

    }
    seat_id_vector.sort();
    let mut my_seat = 0;
    for i in 0..seat_id_vector.len() {
        if i+1 < seat_id_vector.len() {
            let current_value = seat_id_vector[i];
            let next_value = seat_id_vector[i+1];
            if next_value - current_value != 1 {
                println!("{} {}", seat_id_vector[i], seat_id_vector[i+1]);
                my_seat = seat_id_vector[i+1] - 1;
                break;
            }
        }
        
    }
    println!("{}", my_seat);
    Ok(())
}

fn fifth_problem_get_row(row_identifier: char, mut index_array:[u32; 2])-> [u32; 2] {
    let mut partition_value = (index_array[1] - index_array[0]) / 2;
    
    if row_identifier == 'B' || row_identifier == 'R' {
        index_array[0] += partition_value + 1;
    } else {
        index_array[1]  = index_array[0] + partition_value;
    }


    return index_array;
}