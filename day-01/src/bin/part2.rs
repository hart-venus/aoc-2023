fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 
fn preprocess_input(input: &str) -> String {
    let mut new_string: String = input.to_string();
    let replace_pairs: [(&str, &str); 9] = [
        ("one", "1ne"),
        ("two", "2wo"),
        ("three", "3hree"),
        ("four", "4our"),
        ("five", "5ive"),
        ("six", "6ix"),
        ("seven", "7even"),
        ("eight", "8ight"),
        ("nine", "9ine")
    ];
    // this function should be left->right, with no priorities
    // eightwothree -> 8wo3
    // for each index of the array, if the substring that starts at that index
    // starts with the first element of the tuple, replace it with the second
    // using replacen 
    let mut i: usize = 0;
    
    while i < new_string.len() {
        for pair in replace_pairs.iter() {
            if new_string[i..].starts_with(pair.0) {
                // replace the i index with the first character of the second element of the tuple
                new_string = new_string.replacen(pair.0, pair.1, 1);
                // break the loop
                break;
            }
        }
        i += 1;
    }

    // return a string slice that is not owned by the function
    return new_string;
}

fn part2(input: &str) -> String {
    let mut first_digit_flag ;
    let mut running_sum:u64 = 0;
    let mut first_digit = 0;
    let mut last_digit = 0;
    for line in input.lines() {
        // for every character in line
        let preprocessed_line = preprocess_input(line);
        first_digit_flag = false;
        for c in preprocessed_line.chars() {
            // if character is a digit
            if c.is_digit(10) {
                // if no first digit has been found yet
                // set this to be the first digit
                if !first_digit_flag {
                    first_digit = c.to_string().parse::<u64>().unwrap();
                    first_digit_flag = true;
                }
                // this is the last digit
                last_digit = c.to_string().parse::<u64>().unwrap();
            }
        }
        running_sum += last_digit + first_digit*10;
    }
    running_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let output = part2(input);
        assert_eq!(output, "281");
    }
}
// 54433 is too low
// 54533 is too high