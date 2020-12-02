use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    first_problem_first_part();
}


fn first_problem_first_part() -> io::Result<()> {

    let mut file = File::open("./data/1.txt")?;
    // let reader = BufReader::new(file);
    let mut first_number: Option<u32> = None;
    let mut second_number: Option<u32> = None;

    let mut numbers = Vec::new();
    file.read_to_end(&mut numbers).expect("Unable to read data");

    let mut product: u32 = 1;
    let numbers_length = number.len()

    for (i, x) in numbers.iter().enumerate() {
        println!("Item {} = {}", i, x);
    }
    
    // for line in reader.lines() {
    //     let mut sum = 0;

       
    //     let i = line?;
    //     let number: u32 = i.parse::<u32>().unwrap();


    //     let digits: Vec<_> = number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

    //     for digit in &digits {
    //         sum += digit;
    //     }


    //     if sum == 2020 {
    //         if first_number.is_none() {
    //             first_number.get_or_insert(number);
    //         } else {
    //             second_number.get_or_insert(number);
    //         }
    //     }

    //     if !first_number.is_none() && !second_number.is_none() {
    //         product = first_number.unwrap() * second_number.unwrap();
    //     }

    // }
    println!("Question 1a {}", product);
    Ok(())
}
