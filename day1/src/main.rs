use std::{fs, collections::HashMap};

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Should have been able to read the file");
    
    // Give explicit type since type inference does not yield unique possibility
    // Get lines from file an return iterator
    let lines  = contents.lines();

    // Reduce result of each line into final sum
    let result = lines.fold(0, |acc: i32, x: &str| { 
        //// PART 1
        // acc + get_number_from_line_part_1(x) 
        //// PART 2
        acc + get_number_from_line_part_2(x) 
    });

    println!("{result}")
}

fn create_number_from_strings(first: &str, last: &str) -> i32 {
    // TIL this format! macro that can take in various types as arguments (as long as they
    // implement the Display trait) and format them into strings. It's used in println!.
    let string_num = format!("{}{}", first, last);
    // Parse string into i32 and just assume it's always possible.
    string_num.parse::<i32>().unwrap()
}

//
// ** PART 1! **
//

fn _get_number_from_line_part_1(string: &str) -> i32 {
    // Filter string so we have only the numbers left
    // TIL .is_digit (base10 is used here)
    // Use .collect to format iterator of characters into a String collection
    let numbers: String = string.chars().filter(|c| c.is_digit(10)).collect();

    if numbers.len() == 1 {
        return create_number_from_strings(&numbers, &numbers)
    } else {
        let last_char_i = numbers.len() - 1;
        return create_number_from_strings(&numbers[0..1], &numbers[last_char_i..])
    }
}

//
// ** PART 2! **
//

// Can also use &str type to annotate, but &'static str is more specific to denote its static global lifetime
const NUMBERS: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_number_from_line_part_2(string: &str) -> i32 {
    parse_num_map(create_num_map(string))
}

fn create_num_map(string: &str) -> HashMap<usize, Vec<usize>> {
    // Use usize instead of &usize here to ensure that the HashMap's lifetime aligns with the keys
    // (int_num) used in the loop. usize implements the Copy trait whereas &usize is a reference
    // that does not, and has a limited lifetime.
    let mut number_to_indices: HashMap<usize, Vec<usize>> = HashMap::new();

    // Check each number, both integer and word, in the NUMBERS list
    // Use enumerate to get the index in addition to the value
    for (i, str_num) in NUMBERS.iter().enumerate() {
        // Grab the correct integer to represent the word
        let int_num = i + 1;

        // Check if the string contains either the letter or number
        // TIL match_indices! Returns tuples of (index, match_value) for some reason so it needs
        // to be mapped.
        let str_match_indices: Vec<usize> = string.match_indices(str_num).map(|(i, _)| i).collect();
        let num_match_indices: Vec<usize> = string.match_indices(&int_num.to_string()).map(|(i, _)| i).collect();

        let mut all_match_indices = [str_match_indices, num_match_indices].concat();
        
        if all_match_indices.len() > 0 {
            all_match_indices.sort();
            // Insert ordered indices into HashMap by number!
            number_to_indices.insert(int_num, all_match_indices);
        }
    }

    number_to_indices
}

fn parse_num_map(map: HashMap<usize, Vec<usize>>) -> i32 {
    // Create tuples of number and index to compare
    let mut first_num: (usize, usize) = (0, usize::MAX);
    let mut last_num: (usize, usize) = (0, usize::MIN);

    // Iterate through the hashmap! I guess it inherently has an iterator?
    for (key, indices) in map {
        // Retrieve highest and lowest instance of that number
        let first_i = indices.first().unwrap();
        let last_i = indices.last().unwrap();

        // Compare values without cloning by dereferencing
        if *first_i < first_num.1 {
            first_num = (key, *first_i);
        }
        if *last_i >= last_num.1 {
            last_num = (key, *last_i);
        }
    }
    
    // Making these strings to reuse the method from part 1...
    create_number_from_strings(&first_num.0.to_string(),&last_num.0.to_string())
}
