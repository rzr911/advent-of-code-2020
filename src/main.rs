use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

use regex;
use fancy_regex::Regex;
fn main() {
    use std::time::Instant;
    let now = Instant::now();
    // first_problem_first_part();
    // first_problem_second_part();
    // second_problem_first_part();
    // second_problem_second_part();
    // third_problem_first_part();
    // third_problem_second_part();
    // fourth_problem_first_part();
    // fourth_problem_second_part();

    // fifth_problem_first_part();
    // fifth_problem_second_part();
    // sixth_problem_first_part();
    // sixth_problem_second_part();

    

    // seventh_problem_first_part();

    // seventh_problem_second_part();

    // eight_problem_first_part();
    // eight_problem_second_part();

    // ninth_problem_first_part();
    // ninth_problem_second_part();

    // tenth_problem_first_part();
    // tenth_problem_second_part();
    
    // eleventh_problem_first_part();
    // eleventh_problem_second_part();

    twelth_problem_first_part();

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
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

        let captures =  re.captures(string);
        let wrapped_captures = &captures.unwrap();

        if wrapped_captures.is_some() {
            lower_limit = wrapped_captures.as_ref().unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
            upper_limit = wrapped_captures.as_ref().unwrap().get(2).unwrap().as_str().parse::<u32>().unwrap();

            eligible_character = wrapped_captures.as_ref().unwrap().get(3).unwrap().as_str();
            password_string = wrapped_captures.as_ref().unwrap().get(4).unwrap().as_str();

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

        let captures =  re.captures(string);
        let wrapped_captures = &captures.unwrap();
        
        if wrapped_captures.is_some() {
            lower_index = wrapped_captures.as_ref().unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
            upper_index = wrapped_captures.as_ref().unwrap().get(2).unwrap().as_str().parse::<u32>().unwrap();

            eligible_character = wrapped_captures.as_ref().unwrap().get(3).unwrap().as_str();
            password_string = wrapped_captures.as_ref().unwrap().get(4).unwrap().as_str();

            let password_string_vector: Vec<char> = password_string.chars().collect();

            if password_string_vector.get(lower_index as usize -1) == Some(&eligible_character.chars().nth(0).unwrap()) {
                is_valid = !is_valid;
            }
            

            if password_string_vector.get(upper_index as usize -1) == Some(&eligible_character.chars().nth(0).unwrap()) {
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


        while true {
            
            if (down_index >= location_graph.len()) {
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
                is_valid = false;
            }

            let hcl = wrapped_captures.as_ref().unwrap().get(15).unwrap().as_str();

            if !fourth_problem_is_height_valid(hgt_value, hgt_unit) {
                is_valid = false;
            }

            if !if_string_in_vector(ecl, &valid_ecl_values) {
                is_valid = false;
            }

            if byr < 1920 || byr > 2002 || iyr < 2010 || iyr > 2020 || eyr < 2020 || eyr > 2030 {
                is_valid = false;
            }

            if is_valid {
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

fn sixth_problem_first_part() -> io::Result<()> {
    let file = File::open("./data/6.txt")?;
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

    let mut sum_of_answers:i32 = 0;

    for s in passport_data {
        let mut unique_answers_set = HashSet::new();
        let answers:Vec<char> = s.chars().collect();

        for answer in answers {
            if answer != ' ' {
                unique_answers_set.insert(answer);
            }
        }
        
        sum_of_answers += unique_answers_set.len() as i32;

    }

    println!("{}", sum_of_answers);
    Ok(())
}


fn sixth_problem_second_part() -> io::Result<()> {
    let file = File::open("./data/6.txt")?;
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

    let mut sum_of_answers:i32 = 0;

    for s in passport_data {

        let mut split = s.split(" ").collect::<Vec<_>>();
        
        if split.len() == 1 {
            let answers:Vec<char> = s.chars().collect();
            sum_of_answers+=answers.len() as i32;
        } else {
            sum_of_answers+=sixth_problem_count_common_characters(&split);
        }

    }

    println!("{}", sum_of_answers);
    Ok(())
}


fn sixth_problem_count_common_characters(vector: &Vec<&str>) -> i32 {
    
        let mut first_answer_set: HashSet<_> = vector[0].chars().collect();
        let mut intersection: HashSet<_> = first_answer_set;
        
        for j in 0+1..vector.len() {
            let current_index_answer_set: HashSet<_> = vector[j].chars().collect();
            intersection = intersection.intersection(&current_index_answer_set).cloned().collect();
        }

    return intersection.len() as i32;
    
}


fn seventh_problem_first_part() -> io::Result<()> {
    let file = File::open("./data/7.txt")?;
    let reader = BufReader::new(file);

    let mut luggage_rules:Vec<String> = vec![];
    let mut index = 0;

    let mut line_string:String = "".to_string();

    // let re = Regex::new(r"(\w+ \w+) bags contain \d+ (\w+ \w+) bag.?,?.? ?[\d+]? ?(\w+ \w+)?").unwrap();
    let re = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();

    let contained_bag_regex = Regex::new(r"(\w+ \w+) bag").unwrap();

    let mut luggage_rules_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut final_bag_containers:HashSet<String> = HashSet::new();

    

    for line in reader.lines() {
        let string:String = line?;
        line_string = string.clone();
        
        let captures =  re.captures(&line_string);
        luggage_rules.push(string.to_string());
        let mut is_valid = true;

      
        let unwrapped_captures = &captures.unwrap();

        let mut holder:String = "".to_string();
        holder = unwrapped_captures.as_ref().unwrap().get(1).unwrap().as_str().to_string();
        let second_string = unwrapped_captures.as_ref().unwrap().get(2).unwrap().as_str().to_string();
        let mut start = 0;
        let mut contained_bags_vector:Vec<String> = vec![];

        while let Some (m) = contained_bag_regex.captures_from_pos (&second_string, start).unwrap() {
            contained_bags_vector.push(m.get (1).unwrap().as_str().to_string());
            start = m.get (0).unwrap().end(); // Or you can use `end` to avoid overlapping matches
        }
        luggage_rules_map.insert(holder, contained_bags_vector);
        
    }   

    let our_bag = "shiny gold";

    for rule in luggage_rules {
        let captures =  re.captures(&rule);

        let unwrapped_captures = &captures.unwrap();
        let holder = unwrapped_captures.as_ref().unwrap().get(1).unwrap().as_str().to_string();

        let second_string = unwrapped_captures.as_ref().unwrap().get(2).unwrap().as_str().to_string();
        
        let mut start = 0;

        while let Some (m) = contained_bag_regex.captures_from_pos (&second_string, start).unwrap() {
            let bag = m.get (1).unwrap().as_str().to_string();

            if bag == our_bag {
                final_bag_containers.insert(holder.clone());
                let mut key_vectors = find_keys_for_value(&luggage_rules_map, &holder);
                let mut index = 0;

                    while index < key_vectors.len() {

                        let mut temp_key_vectors = find_keys_for_value(&luggage_rules_map, key_vectors[index]);
                        key_vectors.append(&mut temp_key_vectors.clone());
                        final_bag_containers.insert(key_vectors[index].to_string().clone());
                        
                       
                        index+=1;
                    }
                
            }

            start = m.get (0).unwrap().end(); // Or you can use `end` to avoid overlapping matches
        }

        if unwrapped_captures.is_some() {
            let contained_bags = unwrapped_captures.as_ref().unwrap().iter();
            let mut start = 0;
            let mut contained_bags_vector:Vec<String> = vec![];
            let mut holder:String = "".to_string();

            for bag in contained_bags {

                if bag.is_some() {
                    
                    if (start == 1) {
                        holder = bag.as_ref().unwrap().as_str().to_string();
                    }

                    if start > 1 {
                        
                        if bag.as_ref().unwrap().as_str() == our_bag {
                            final_bag_containers.insert(holder.clone());
                            let mut key_vectors = find_keys_for_value(&luggage_rules_map, &holder);
                            let mut index = 0;

                                while(index < key_vectors.len()) {

                                    let mut temp_key_vectors = find_keys_for_value(&luggage_rules_map, key_vectors[index]);
                                    key_vectors.append(&mut temp_key_vectors.clone());
                                    final_bag_containers.insert(key_vectors[index].to_string().clone());
                                    
                                   
                                    index+=1;
                                }
                            
                        }
                    }
                }

                start+=1;
            }
        }
    }

    // println!("{:?}", final_bag_containers);

    println!("{:?}", final_bag_containers.len());
    

    Ok(())
}


fn find_keys_for_value<'a>(map: &'a HashMap<String, Vec<String>>, value: &str) -> Vec<&'a String> {
    let table = vec![value];

    let mut res: Vec<&String> =   map.iter() .filter_map(|(key, values)|  if values.contains(&value.to_string()) { Some(key) } else { None }).collect();
  
    return res;
}

fn seventh_problem_second_part() -> io::Result<()> {
    let file = File::open("./data/7.txt")?;
    let reader = BufReader::new(file);

    let mut luggage_rules:Vec<String> = vec![];
    let mut index = 0;

    let mut line_string:String = "".to_string();

    // let re = Regex::new(r"(\w+ \w+) bags contain \d+ (\w+ \w+) bag.?,?.? ?[\d+]? ?(\w+ \w+)?").unwrap();
    let re = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();

    let contained_bag_regex = Regex::new(r"(\d+ \w+ \w+) bag").unwrap();

    let contained_bag_regex_with_separate_count_match = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    let mut luggage_rules_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut final_bag_containers:HashSet<String> = HashSet::new();

    

    for line in reader.lines() {
        let string:String = line?;
        line_string = string.clone();
        
        let captures =  re.captures(&line_string);
        luggage_rules.push(string.to_string());
        let mut is_valid = true;

      
        let unwrapped_captures = &captures.unwrap();

        let mut holder:String = "".to_string();
        holder = unwrapped_captures.as_ref().unwrap().get(1).unwrap().as_str().to_string();
        let second_string = unwrapped_captures.as_ref().unwrap().get(2).unwrap().as_str().to_string();
        let mut start = 0;
        let mut contained_bags_vector:Vec<String> = vec![];

        while let Some (m) = contained_bag_regex.captures_from_pos (&second_string, start).unwrap() {
            contained_bags_vector.push(m.get (1).unwrap().as_str().to_string());
            start = m.get (0).unwrap().end(); // Or you can use `end` to avoid overlapping matches
        }

        luggage_rules_map.insert(holder, contained_bags_vector);
        
    }   
    let our_bag = "shiny gold";
    let mut total_count: Option<i32>= get_count_of_bags(&luggage_rules_map, "shiny gold".to_string());

    println!("{:?}", total_count.unwrap());
    

    Ok(())
}

fn get_count_of_bags(luggage_rules_map: &HashMap<String, Vec<String>>, bag: String) -> Option<i32> {
    let our_bag = bag.clone();
    let mut total = 0;
    let contained_bag_regex_with_separate_count_match = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    let key_vectors: &mut Vec<String> = &mut luggage_rules_map.get(&our_bag).unwrap().clone();
    

    for i in 0..key_vectors.len() {
        let bag_with_count = key_vectors[i].to_string().clone();
        let captures = contained_bag_regex_with_separate_count_match.captures(&bag_with_count);
        let unwrapped_captures = &captures.unwrap();
        
        if unwrapped_captures.is_some() {
            let count = unwrapped_captures.as_ref().unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
            let bag = unwrapped_captures.as_ref().unwrap().get(2).unwrap().as_str();
            let optional_count = get_count_of_bags(luggage_rules_map, bag.to_string());

            if optional_count.is_some() {
                total += count + count * optional_count.unwrap();
            }
        }
        
    }

    return Some(total);
}

fn eight_problem_first_part() -> io::Result<()> {
    let file = File::open("./data/8.txt")?;
    let reader = BufReader::new(file);

    let mut instructions:Vec<String> = vec![];
    let mut index = 0;

    let mut parsed_indexes:Vec<i32> = vec![];

    for line in reader.lines() {
        let string:&str = &line?;
        instructions.push(string.to_string());
    }
    
    let mut current_index =  0 as i32;

    let mut accumulator = 0 as i32;

    let instruction_regex = Regex::new(r"(nop|acc|jmp) (\+|-)(\d+)").unwrap();
    
    
    let instruction_cycle_response = eight_problem_execute_instruction_cycle(instructions);
    println!("{}", instruction_cycle_response.1);
    Ok(())
}

fn eight_problem_second_part() -> io::Result<()> {
    let file = File::open("./data/8.txt")?;
    let reader = BufReader::new(file);

    let mut instructions:Vec<String> = vec![];
    let mut index = 0;
    let mut accumulator = 0;

    let mut parsed_indexes:Vec<i32> = vec![];

    let mut jmp_indexes: Vec<i32> = vec![];
    let mut nop_indexes: Vec<i32> = vec![];


    for line in reader.lines() {
        let string:&str = &line?;

        if string.contains("nop") {
            nop_indexes.push(index);
        } else if string.contains("jmp") {
            jmp_indexes.push(index);
        }

        instructions.push(string.to_string());
        index+=1;
    }

    let mut is_infinite_loop = false;

    while !is_infinite_loop {

        for i in 0..nop_indexes.len() {
            let mut modified_instruction_vector = instructions.to_vec();
            modified_instruction_vector[jmp_indexes[i] as usize] = modified_instruction_vector[jmp_indexes[i] as usize].replace("nop", "jmp");
            
            let mut response = eight_problem_execute_instruction_cycle(modified_instruction_vector);
            
            if response.0 {
                accumulator += response.1;
                is_infinite_loop = response.0;
                break;
            }
        }

        if is_infinite_loop {
            break;
        }

        for i in 0..jmp_indexes.len() {
            let mut modified_instruction_vector = instructions.to_vec();
            modified_instruction_vector[jmp_indexes[i] as usize] = modified_instruction_vector[jmp_indexes[i] as usize].replace("jmp", "nop");
            
            let mut response = eight_problem_execute_instruction_cycle(modified_instruction_vector);
            
            
            if response.0 {
                accumulator = response.1;
                is_infinite_loop = response.0;
                break;
            }
        }
        
    }
    
    println!("{} {}", is_infinite_loop , accumulator);

    Ok(())
}


fn eight_problem_execute_instruction_cycle(instructions:Vec<String>) -> (bool, i32) {
    let instruction_regex = Regex::new(r"(nop|acc|jmp) (\+|-)(\d+)").unwrap();

    let mut current_index =  0 as i32;

    let mut accumulator = 0 as i32;

    let mut parsed_indexes:Vec<i32> = vec![];

    while !parsed_indexes.contains(&current_index) {
        
        parsed_indexes.push(current_index);
        let current_instruction = &instructions[current_index as usize];

        let captures = instruction_regex.captures(current_instruction);
        let unwrapped_captures = &captures.unwrap();        
        
        if unwrapped_captures.is_some() {
            let count = unwrapped_captures.as_ref().unwrap().get(3).unwrap().as_str().parse::<i32>().unwrap();
            let instruction = unwrapped_captures.as_ref().unwrap().get(1).unwrap().as_str();
            let sign = unwrapped_captures.as_ref().unwrap().get(2).unwrap().as_str();

            if current_index == (instructions.len() - 1) as i32 {
                
                if instruction == "acc" {

                    if sign == "+" {
                        accumulator+=count;
                    } else {
                        accumulator-=count;
                    }
                }
                
                return (true, accumulator);
            }

            if instruction == "jmp" {

                if sign == "+" {
                    current_index+=count;
                } else {
                    current_index-=count;
                }
            } else {
            
                if instruction == "acc" {

                    if sign == "+" {
                        accumulator+=count;
                    } else {
                        accumulator-=count;
                    }
                }
            current_index+=1;
            }

            
        }

    }

    return (false, accumulator);
}


fn ninth_problem_first_part() -> io::Result<()> {
    let file = File::open("./data/9.txt")?;
    let reader = BufReader::new(file);

    let mut preamble:Vec<i32> = vec![];
    let preamble_length:usize = 25;
    let mut index = 0;

    let mut exceptional_number: i32 = 0;

    for line in reader.lines() {
        let string:&str = &line?;
        preamble.push(string.to_string().parse::<i32>().unwrap());
        
        if index > preamble_length - 1 {
            let modified_vector:Vec<i32> = preamble[index - preamble_length..index].to_vec();
            let response = ninth_problem_check_if_number_sum_of_any_two_in_preamble(modified_vector, preamble[index]);

            if !response {
                exceptional_number = preamble[index];
                break;
            }
        } 

        index+=1;
    }

    println!("{}", exceptional_number);

    Ok(())
}

fn ninth_problem_second_part() -> io::Result<()> {
    let file = File::open("./data/9.txt")?;
    let reader = BufReader::new(file);

    let mut preamble:Vec<i32> = vec![];
    let preamble_length:usize = 25;
    let mut index = 0;

    let mut exceptional_number: i32 = 0;
    let mut exceptional_index: usize = 0;

    for line in reader.lines() {
        let string:&str = &line?;
        preamble.push(string.to_string().parse::<i32>().unwrap());
        
        if index > preamble_length - 1 {
            let modified_vector:Vec<i32> = preamble[index - preamble_length..index].to_vec();
            let response = ninth_problem_check_if_number_sum_of_any_two_in_preamble(modified_vector, preamble[index]);

            if !response {
                exceptional_number = preamble[index];
                exceptional_index = index;
                break;
            }
        } 

        index+=1;
    }

    let mut is_sum_equal = false;

    let mut range: (usize, usize) = (0, 0);

    for i in 0..exceptional_index {
        let mut sum: i32 = preamble[i];

        for j in i+1..exceptional_index {
            sum += preamble[j];
            
            if sum == exceptional_number {
                range = (i, j);
                is_sum_equal = true;
                break;
            }
        }

        if is_sum_equal {
            break;
        }
    }

    let mut smallest_number: i32 = 0;
    let mut largest_number: i32 = 0;

    for i in range.0..range.1+1 {

        if i == range.0 {
            smallest_number = preamble[i];
            largest_number = preamble[i];
        }

        if smallest_number > preamble[i] {
            smallest_number = preamble[i];
        }

        if largest_number < preamble[i] {
            largest_number = preamble[i];
        }
    }

    println!("{:?} {} + {} = {} {}", range, smallest_number, largest_number, smallest_number + largest_number, exceptional_number);

    Ok(())
}

fn ninth_problem_check_if_number_sum_of_any_two_in_preamble(preamble: Vec<i32>, number: i32) -> bool {
    let mut is_equal = false;
    
    for i in 0..preamble.len() {
        
        for j in i+1..preamble.len() {

            if i!=j &&  preamble[i]+preamble[j] == number {
                is_equal = true;
                break;
            }
        }

        if is_equal {
            break;
        }
    }

    return is_equal;
}

fn tenth_problem_first_part() -> io::Result<()> {
    let file = File::open("./data/10.txt")?;
    let reader = BufReader::new(file);

    let mut adapters:Vec<i32> = vec![];
    let mut index = 0;

    let mut count_of_1_jolt_difference = 0;
    let mut count_of_3_jolt_difference = 0;

    for line in reader.lines() {
        let string:&str = &line?;
        adapters.push(string.to_string().parse::<i32>().unwrap());
        
        index+=1;
    }


    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len()-1] + 3);

    println!("{:?}", adapters);

    let mut prev_jolt = 0;
    

    for i in  0..adapters.len() {

        if i != 0 {
            prev_jolt = adapters[i-1];
            let current_jolt = adapters[i];
            let difference = current_jolt - prev_jolt;
            if difference == 1 {
                count_of_1_jolt_difference+=1;
            } else if difference == 3 {
                count_of_3_jolt_difference+=1;
            }

            // println!("{} {} {}", current_jolt, prev_jolt, difference)
        }
    }

    println!("{} {} {}", count_of_1_jolt_difference , count_of_3_jolt_difference, count_of_1_jolt_difference * count_of_3_jolt_difference);
    Ok(())
}

fn tenth_problem_second_part() -> io::Result<()> {
    let file = File::open("./data/test.txt")?;
    let reader = BufReader::new(file);

    let mut adapters:Vec<i32> = vec![];
    let mut index = 0;

    let mut count_of_1_jolt_difference = 0;
    let mut count_of_3_jolt_difference = 0;
    let mut all_possible_combinations: HashSet<Vec<i32>> = HashSet::new();;

    for line in reader.lines() {
        let string:&str = &line?;
        adapters.push(string.to_string().parse::<i32>().unwrap());
        
        index+=1;
    }

    adapters.push(4);
    adapters.sort();
    adapters.push(adapters[adapters.len()-1] + 3);

    println!("{:?}", adapters);

    let mut prev_jolt = 0;

    tenth_problem_second_part_get_all_possible_combinations(adapters);
    Ok(())
}

fn tenth_problem_second_part_get_all_possible_combinations(adapters: Vec<i32>) -> Vec<Vec<i32>> {
    let mut all_possible_combinations:Vec<Vec<i32>> = vec![];
    let mut count:i32 = 1;
    let mut considered_end_indices: HashSet<i32> = HashSet::new();

    for i in 0..adapters.len() {

        if i < adapters.len() - 3 && adapters[i+3] - adapters[i] == 3 {

            if tenth_problem_check_if_index_lower_than_passed_index_exists(considered_end_indices.clone(), adapters[i] as i32) {
                count+=2;
                println!("{} {} {}", count, adapters[i+3], adapters[i]);
            } else {
                count*=4;
            }
            considered_end_indices.insert(adapters[i+3]);

        }
        
        if i < adapters.len() - 3 && adapters[i+3] - adapters[i] == 3 {

            if tenth_problem_check_if_index_lower_than_passed_index_exists(considered_end_indices.clone(), adapters[i] as i32) {
                count+=2;
                println!("{} {} {}", count, adapters[i+3], adapters[i]);
            } else {
                count*=4;
            }
            considered_end_indices.insert(adapters[i+3]);

        } else if (i < adapters.len() - 2 && adapters[i+2] - adapters[i] < 4) {
            println!("Difference between i and i + 2 {} {}", adapters[i], adapters[i+2]);
            
            if !considered_end_indices.contains(&adapters[i+2]) {
                count*=2;

                if tenth_problem_check_if_index_lower_than_passed_index_exists(considered_end_indices.clone(), adapters[i] as i32) {
                    count-=1;
                    println!("{} {} {}", count, adapters[i+2], adapters[i]);
                }
                considered_end_indices.insert(adapters[i+2]);
                println!("{:?}", considered_end_indices);
            }
        } 

    }
    println!("{} ",count);
    return all_possible_combinations;
}

fn tenth_problem_check_if_index_lower_than_passed_index_exists(considered_end_indices: HashSet<i32>, index: i32) -> bool {

    for i in &considered_end_indices {

        if index < *i {
            return true;
        }
    }
    return false;
}


fn eleventh_problem_first_part() -> io::Result<()> {
    let file = File::open("./data/11.txt")?;
    let reader = BufReader::new(file);

    let mut layout:Vec<Vec<char>> = vec![];
    let mut index = 0;
    let mut occupied_seat_count = 0;

    for line in reader.lines() {
        let string:&str = &line?;
        let char_vec: Vec<char> = string.chars().collect();
        layout.push(char_vec);
    }
    let mut is_modified = true;

    while is_modified {
        let res = eleventh_problem_first_part_apply_seat_rules(layout.clone());
        is_modified = res.1;
        layout = res.0;
    }
    


    for i in 0..layout.len() {
        for j in 0..layout[i].len() {

            if layout[i][j] == '#' {
                occupied_seat_count+=1;
            }
        }
        
    }
    println!("{}", occupied_seat_count);
    Ok(())
}

fn eleventh_problem_first_part_apply_seat_rules(layout: Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut modified_layout = layout.clone();
    
    let mut is_modified = false;


    for i in 0..modified_layout.len() {
        for j in 0..modified_layout[i].len() {
            let adjacent_index_vector = eleventh_problem_first_part_get_adjacent_indices_based_on_current_index((i,j), modified_layout[i].len(), modified_layout.len());
                
            if layout[i][j] == 'L' {
                let mut are_adjacent_seats_occupied = false;

                for index_tuple in adjacent_index_vector {
                    
                    if layout[index_tuple.0][index_tuple.1] == '#' {
                        are_adjacent_seats_occupied = true;
                        
                    }
                }

                if !are_adjacent_seats_occupied {
                    modified_layout[i][j] = '#';
                    is_modified = true;
                }
                
            } else if layout[i][j] == '#' {
                let mut occupied_seat_count = 0;

                if i==1 && j==8 {
                    println!("{:?}", adjacent_index_vector);
                }

                for index_tuple in adjacent_index_vector {
                    
                    if layout[index_tuple.0][index_tuple.1] == '#' {
                        occupied_seat_count+=1;
                    }
                }

                if occupied_seat_count >= 4 {
                    modified_layout[i][j] = 'L';
                    is_modified = true;
                }
            }
        }
    }

    return (modified_layout, is_modified);
}

fn eleventh_problem_first_part_get_adjacent_indices_based_on_current_index(current_index: (usize, usize), row_length: usize, column_length: usize) -> Vec<(usize, usize)> {

    let mut adjacent_index_tuple_vector:Vec<(usize, usize)> = vec![];
    let i = current_index.0;
    let j = current_index.1;

    
    if i == 0 && j==0 {

        if j+1 < row_length {
            adjacent_index_tuple_vector.push((i, j+1));
        }

        if i+1 < column_length {
            adjacent_index_tuple_vector.push((i+1, j));
        }

        if i+1 < column_length && j+1 < row_length {
            adjacent_index_tuple_vector.push((i+1, j+1));
        }

    } else if i > 0 && j==0 {

        adjacent_index_tuple_vector.push((i-1, j));
        
        if j+1 < row_length {
            adjacent_index_tuple_vector.push((i-1, j+1));

            if i+1 < column_length {
                adjacent_index_tuple_vector.push((i+1, j+1));
            }
            adjacent_index_tuple_vector.push((i, j+1));
        }

        if i+1 < column_length {
            adjacent_index_tuple_vector.push((i+1, j));
        }

    } else if i == 0 && j > 0 {

        adjacent_index_tuple_vector.push((i, j-1));

        if j+ 1 < row_length {
            adjacent_index_tuple_vector.push((i, j+1));
        }

        if i+1 < column_length {
            adjacent_index_tuple_vector.push((i+1, j-1));
            adjacent_index_tuple_vector.push((i+1, j));
            
            if j+1 < row_length {
                adjacent_index_tuple_vector.push((i+1, j+1));
            }
        }

    } else {
        adjacent_index_tuple_vector.push((i-1, j-1));
        adjacent_index_tuple_vector.push((i-1, j));
        
        if j+1 < row_length {
            adjacent_index_tuple_vector.push((i-1, j+1));
            adjacent_index_tuple_vector.push((i, j+1));
        }

        adjacent_index_tuple_vector.push((i, j-1));

        if i+1 < column_length  {
            adjacent_index_tuple_vector.push((i+1, j-1));
            adjacent_index_tuple_vector.push((i+1, j));

            if j+1 < row_length {
                adjacent_index_tuple_vector.push((i+1, j+1));
            }
        }
    }
    return adjacent_index_tuple_vector;
}


fn eleventh_problem_second_part() -> io::Result<()> {
    let file = File::open("./data/11.txt")?;
    let reader = BufReader::new(file);

    let mut layout:Vec<Vec<char>> = vec![];
    let mut index = 0;
    let mut occupied_seat_count = 0;

    for line in reader.lines() {
        let string:&str = &line?;
        let char_vec: Vec<char> = string.chars().collect();
        layout.push(char_vec);
    }

    let mut is_modified = true;

    while is_modified {
        let mut res = eleventh_problem_second_part_apply_seat_rules(layout.clone());
        is_modified = res.1;
        layout = res.0;
    }

    
    for i in 0..layout.len() {

        for j in 0..layout[i].len() {
            
            if layout[i][j] == '#' {
                occupied_seat_count+=1;
            }
        }
        println!("");
    }

    println!("Occupied seat count {}", occupied_seat_count);
    Ok(())
}

fn eleventh_problem_second_part_apply_seat_rules(layout: Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut modified_layout = layout.clone();
    
    let mut is_modified = false;


    for i in 0..modified_layout.len() {

        for j in 0..modified_layout[i].len() {
                
            if layout[i][j] == 'L' {
                let mut are_nearest_seats_occupied = false;
                
                let are_nearest_seats_occupied = eleventh_problem_second_part_are_nearest_seats_occupied((i,j), layout.clone()).0;
            
                

                if !are_nearest_seats_occupied {
                    modified_layout[i][j] = '#';
                    is_modified = true;
                }
                
            } else if layout[i][j] == '#' {
                let adjacent_index_vector = eleventh_problem_first_part_get_adjacent_indices_based_on_current_index((i,j), modified_layout[i].len(), modified_layout.len());
            
                let mut occupied_seat_count = eleventh_problem_second_part_are_nearest_seats_occupied((i,j), layout.clone()).1;

    
                if occupied_seat_count >= 5 {
                    modified_layout[i][j] = 'L';
                    is_modified = true;
                }
            }
        }
    }

    return (modified_layout, is_modified);
}

fn eleventh_problem_second_part_are_nearest_seats_occupied(index_tuple:(usize, usize), layout: Vec<Vec<char>>) -> (bool,usize) {
    let mut are_nearest_seats_occupied = false;
    let mut count_of_visible_seats:usize = 0;

    // check for nearest vertically down
    for i in index_tuple.0+1..layout.len() {

        
       
        if layout[i][index_tuple.1] == '#' {
            are_nearest_seats_occupied = true;
            
            count_of_visible_seats+=1;
            if index_tuple.0==0 && index_tuple.1==3 {
                println!("{}", are_nearest_seats_occupied);
        
                 println!("{} {}",  i,index_tuple.1);
            }
           
        break;
       } else if layout[i][index_tuple.1] == 'L' {

        break;
        }
    }

     // check for nearest vertically up
    for i in (0..index_tuple.0).rev() {

        if index_tuple.0==9 && index_tuple.1==9 {
            // println!("{} {} value {}",i, index_tuple.1, layout[i][index_tuple.1])
        }

        if layout[i][index_tuple.1] == '#' {
            are_nearest_seats_occupied = true;
            count_of_visible_seats+=1;

            if index_tuple.0==0 && index_tuple.1==3 {
                println!(" {}", are_nearest_seats_occupied);
                println!("{} {}",  i,index_tuple.1);
            }
            break;
        }
        else if layout[i][index_tuple.1] == 'L' {

            break;
        }
        
    }

    // check for nearest horizontally right
    for j in index_tuple.1+1..layout[index_tuple.0].len() {

        if layout[index_tuple.0][j] == '#' {
            are_nearest_seats_occupied = true;
            count_of_visible_seats+=1;

            if index_tuple.0==0 && index_tuple.1==3 {
                println!(" ay ay {}", are_nearest_seats_occupied);
                println!("{} {}",  index_tuple.0,j);
            }
            break;
        }

        else if layout[index_tuple.0][j] == 'L' {
            break;
         }
    }

    // check for nearest horizontally left
    for j in (0..index_tuple.1).rev() {

        if index_tuple.0==0 && index_tuple.1==9 {
            // println!("{} {} value {}",index_tuple.0, j, layout[index_tuple.0][j])
        }

        if layout[index_tuple.0][j] == '#' {
            are_nearest_seats_occupied = true;
            count_of_visible_seats+=1;

            if index_tuple.0==0 && index_tuple.1==3 {
                println!(" ay ay {}", are_nearest_seats_occupied);
                println!("{} {}",  index_tuple.0,j);
            }
           
        break;
        } else if layout[index_tuple.0][j] == 'L' {

        break;
        }
    }

    // Upwards left
    let mut i = index_tuple.0;
    let mut j = index_tuple.1;

    if i != 0 && j!=0 {
        i-=1;
        j-=1;

        while i >= 0 && j >=0 {
        
            if index_tuple.0==5 && index_tuple.1==4 {
                // println!("{} {} value {}",i,j, layout[i][j])
            }
    
            if layout[i][j] == '#' {
                are_nearest_seats_occupied = true;
                count_of_visible_seats+=1;
                
                if index_tuple.0==0 && index_tuple.1==3 {
                    println!("{}", are_nearest_seats_occupied);
                    println!("{} {}",  i,j);
                }

                break;
            } else if layout[i][j] == 'L' {

                break;
            }
    
            if i == 0 || j == 0 {
                break;
            } 
            i-=1;
            j-=1;
        }

        let mut i = index_tuple.0 - 1;
        let mut j = index_tuple.1 - 1;
    }
    
    
    // downwards left
    i = index_tuple.0;
    j = index_tuple.1;
    
    if i != layout.len() - 1 && j!=0 {
        i+=1;
        j-=1;

        while i < layout.len() && j >=0 {

            if index_tuple.0==8 && index_tuple.1==5 {
                // println!("{} {} value {}",i,j, layout[i][j])
            }
            
            if layout[i][j] == '#' {
                are_nearest_seats_occupied = true;
                count_of_visible_seats+=1;
                
                if index_tuple.0==0 && index_tuple.1==3 {
                    println!("{}", are_nearest_seats_occupied);
                    println!("{} {}",  i,j);
                }

                break;
            } else if layout[i][j] == 'L' {
                break;
            }
            if j == 0 {
                break;
            } 
    
            i+=1;
    
            if j == 0 {
                break;
            } 
            j-=1;
        }
    }
    
    // downwards right
    i = index_tuple.0;
    j = index_tuple.1;

    if i != layout.len() - 1 && j!=layout[i].len() {
        i+=1;
        j+=1;

        while i < layout.len() && j < layout[i].len() {
            
            if layout[i][j] == '#' {
                are_nearest_seats_occupied = true;
                count_of_visible_seats+=1;

                if index_tuple.0==0 && index_tuple.1==3 {
                    println!("{}", are_nearest_seats_occupied);
                    println!("{} {}",  i,j);
                }

                break;
            } else if layout[i][j] == 'L' {
                break;
            }

            i+=1;
            j+=1;
        }
    }
    
    //upwards right

    i = index_tuple.0;
    j = index_tuple.1;

    if i!=0 && j!=layout[i].len() {
        i-=1;
        j+=1;

        while i >= 0 && j < layout[i].len() {

            if layout[i][j] == '#' {
                are_nearest_seats_occupied = true;
                count_of_visible_seats+=1;

                if index_tuple.0==0 && index_tuple.1==3 {
                    println!("{}", are_nearest_seats_occupied);
                    println!("{} {}",  i,j);
                }
                break;
                // return are_nearest_seats_occupied;
            }  else if layout[i][j] == 'L' {

                break;
            }
    
            if i == 0 {
                break;
            } 
            i-=1;
            j+=1;
        }
    }

    return (are_nearest_seats_occupied, count_of_visible_seats);

}


fn twelth_problem_first_part() ->  io::Result<()>{
    let file = File::open("./data/test.txt")?;
    let reader = BufReader::new(file);

    let mut index = 0;
    let mut occupied_seat_count = 0;

    let location_coordinate = (0, 0);
    let direction = 'E';

    for line in reader.lines() {

        let string:&str = &line?;

        twelth_problem_first_part_parse_action(location_coordinate, direction, string);


    }


    Ok(())
}

fn twelth_problem_first_part_parse_action(mut location: (i32, i32), direction: char, action_string: &str) ->((i32,i32), char) {
    
    let re = Regex::new(r"(N|S|E|W|L|R|F)(\d+)").unwrap();
    let captures =  re.captures(&action_string);

    let unwrapped_captures = &captures.unwrap();
    let action = unwrapped_captures.as_ref().unwrap().get(1).unwrap().as_str();
    let value = unwrapped_captures.as_ref().unwrap().get(2).unwrap().as_str().to_string().parse::<i32>().unwrap();

    if action == 'F'.to_string() {

        if direction == 'E' {
            location.0+=value;
        } else if direction == 'W' {
            location.0-=value;
        } else if direction == 'N' {
            location.1+=value;
        } else if direction == 'S' {
            location.1-=value;
        }
    } else if action == 'N'.to_string() {
        if direction == 'E' {
            location.0+=value;
        } else if direction == 'W' {
            location.0-=value;
        } else if direction == 'N' {
            location.1+=value;
        } else if direction == 'S' {
            location.1-=value;
        }
    }

    println!("{:?}", location);
    return (location, direction)

}
