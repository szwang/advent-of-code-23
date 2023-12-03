use std::fs;
use regex::Regex;

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("should read file successfully");

    // Create a vecs of colors by game [[(r,g,b)]]
    let color_vec: Vec<Vec<(usize, usize, usize)>> = 
        file_content.lines().map(|line| create_color_vec(line)).collect();

    //// Part 1
    // let result = part_1(color_vec);

    //// Part 2
    let result = part_2(color_vec);

    println!("{result}");
}

fn part_1(color_vec: Vec<Vec<(usize, usize, usize)>>) -> usize {
    // Use enumerate here to get the index, which is the game number
    color_vec.iter().enumerate().fold(0, |acc, (i, game_set)| {
        // The i value in enumerate is usize. I originally had i32 types for the vec
        // contents and needed to cast it, which would be:
        // let game_number: i32 = (i as i32) + 1; 
        let game_number = i + 1;
        // TIL .all — returns true if all are true, false if at least one is false
        let all_match = game_set.iter().all(|set| set_is_possible(set));
        if all_match {
            acc + game_number
        } else {
            acc
        }
    })
}

fn part_2(color_vec: Vec<Vec<(usize, usize, usize)>>) -> usize {
    color_vec.iter().fold(0, |total, game_set| {
        let (max_r, max_g, max_b) = 
            /*
                I originally tried destructuring with (set_r, set_g, set_b) without the `&`.
                The & is important because the loop variable in iterators is always a reference!

                So I want to match that in my destructure and tell Rust that it is a reference.
                Without &, it is a type mismatch between &T and T. Through since this tuple is simple  
                and implements the Copy trait, Rust handles things for me even though I'm not
                being explicit.

                For some reason I also needed to do fold((0 as usize, 0 as usize, 0 as usize) without
                the & as well.
             */
            game_set.iter().fold((0, 0, 0), |(max_r, max_g, max_b), &(set_r, set_g, set_b)| {
            /*
                I originally had this (so used to ternaries!)
                let r = if set_r > &max_r { set_r } else { &max_r };
                let g = if set_g > &max_g { set_g } else { &max_g };
                let b = if set_b > &max_b { set_b } else { &max_b };

                (*r, *g, *b)

                but learned about .max, which is more elegant!
            */
            (max_r.max(set_r), max_g.max(set_g), max_b.max(set_b))
        });
        total + (max_r * max_g * max_b)
    })
}

fn set_is_possible((r,g,b): &(usize, usize, usize)) -> bool {
    *r <= 12 && *g <= 13 && *b <= 14
}

fn create_color_vec(line: &str) -> Vec<(usize, usize, usize)> {
    // Match colon and whitespace (\s), capture all characters until end of line (.*)
    let color_regex = Regex::new(r":\s(.*)").unwrap();
    let regex_result = color_regex.captures(line).unwrap();
    let colors_string = regex_result.get(1).unwrap().as_str();
    let string_vec: Vec<String> = colors_string.split("; ").map(str::to_string).collect();
    string_vec.iter().map(|set| get_color_tuple(set)).collect()
}   

fn get_color_tuple(string: &str) -> (usize, usize, usize) {
    let r = get_color_num("red", string);
    let g = get_color_num("green", string);
    let b = get_color_num("blue", string);
    (r, g, b)
}

fn get_color_num(color: &str, text: &str) -> usize {
    // Capture 1 or more (+) numbers (\d) that precede whitespace and color string
    let regex_pattern = format!(r"(\d+)\s*{}", color);
    let re = Regex::new(&regex_pattern).unwrap();
    match re.captures(text) {
        Some(matched_str) => {
            matched_str.get(1).unwrap().as_str().parse::<usize>().unwrap()
        },
        None => 0
    }
}