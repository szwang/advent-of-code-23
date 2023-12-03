use std::{fs, collections::HashMap};
use regex::Regex;

// 12 red cubes, 13 green cubes, and 14 blue cubes

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("should read file successfully");

    // Split lines
    // Get line after colon
    let color_vec: Vec<Vec<(i32, i32, i32)>> = file_content.lines().map(|line| {
        // ["2 red, 2 green", "6 red, 3 green", "2 red, 1 green, 2 blue", "1 red"]
        get_colors_string_vec(line).iter().map(|set| get_color_tuple(set)).collect()
    }).collect();

    println!("{:?}", color_vec);

    let result = color_vec.iter().enumerate().fold(0, |acc, (i, game_set)| {
        let game_number = i + 1;
        // Check to see if r <= 12, g <= 13, b <= 14
        let all_match = game_set.iter().all(|set| set_is_possible(set));
        if all_match {
            return acc + game_number;
        } else {
            return acc;
        }
    });

    println!("{result}");
}

fn set_is_possible((r,g,b): &(i32, i32, i32)) -> bool {
    *r <= 12 && *g <= 13 && *b <= 14
}

fn get_colors_string_vec(line: &str) -> Vec<String> {
    let color_regex = Regex::new(r":\s(.*)").unwrap();
    let regex_result = color_regex.captures(line).unwrap();
    let colors_string = regex_result.get(1).unwrap().as_str();
    colors_string.split("; ").map(str::to_string).collect()
}   

fn get_color_tuple(string: &str) -> (i32, i32, i32) {
    let r = retrieve_number_before("red", string);
    let g = retrieve_number_before("green", string);
    let b = retrieve_number_before("blue", string);
    (r, g, b)
}

fn retrieve_number_before(color: &str, text: &str) -> i32 {
    let regex_pattern = format!(r"(\d+)\s*{}", color);
    let re = Regex::new(&regex_pattern).unwrap();
    match re.captures(text) {
        Some(matched_str) => {
            matched_str.get(1).unwrap().as_str().parse::<i32>().unwrap()
        },
        None => 0
    }
}