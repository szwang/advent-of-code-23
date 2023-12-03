use std::fs;
use regex::Regex;

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("should read file successfully");

    // Create a vec of colors by game [[(r,g,b)]]
    let color_vec: Vec<Vec<(i32, i32, i32)>> = 
        file_content.lines().map(|line| create_color_vec(line)).collect();

    //// Part 1
    // let result = part_1(color_vec);

    //// Part 2
    let result = part_2(color_vec);

    println!("{result}");
}

fn part_1(color_vec: Vec<Vec<(i32, i32, i32)>>) -> i32 {
    color_vec.iter().enumerate().fold(0, |acc, (i, game_set)| {
        let game_number: i32 = (i + 1).try_into().unwrap();
        let all_match = game_set.iter().all(|set| set_is_possible(set));
        if all_match {
            return acc + game_number;
        }
        return acc;
    })
}

fn part_2(color_vec: Vec<Vec<(i32, i32, i32)>>) -> i32 {
    color_vec.iter().fold(0, |acc, game_set| {
        // retrieve largest value of each tuple place in game set
        let largest = game_set.iter().fold((0,0,0), |(acc_r, acc_g, acc_b), (set_r, set_g, set_b)| {
            let r = if set_r > &acc_r { set_r } else { &acc_r };
            let g = if set_g > &acc_g { set_g } else { &acc_g };
            let b = if set_b > &acc_b { set_b } else { &acc_b };

            (*r, *g, *b)
        });
        acc + (largest.0 * largest.1 * largest.2)
    })
}

fn set_is_possible((r,g,b): &(i32, i32, i32)) -> bool {
    *r <= 12 && *g <= 13 && *b <= 14
}

fn create_color_vec(line: &str) -> Vec<(i32, i32, i32)> {
    let color_regex = Regex::new(r":\s(.*)").unwrap();
    let regex_result = color_regex.captures(line).unwrap();
    let colors_string = regex_result.get(1).unwrap().as_str();
    let string_vec: Vec<String> = colors_string.split("; ").map(str::to_string).collect();
    string_vec.iter().map(|set| get_color_tuple(set)).collect()
}   

fn get_color_tuple(string: &str) -> (i32, i32, i32) {
    let r = get_color_num("red", string);
    let g = get_color_num("green", string);
    let b = get_color_num("blue", string);
    (r, g, b)
}

fn get_color_num(color: &str, text: &str) -> i32 {
    let regex_pattern = format!(r"(\d+)\s*{}", color);
    let re = Regex::new(&regex_pattern).unwrap();
    match re.captures(text) {
        Some(matched_str) => {
            matched_str.get(1).unwrap().as_str().parse::<i32>().unwrap()
        },
        None => 0
    }
}