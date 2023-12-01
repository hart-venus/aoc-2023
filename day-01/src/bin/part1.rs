fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 

fn part1(input: &str) -> String {
    // for every line in input
    let mut first_digit_flag ;
    let mut running_sum:u64 = 0;
    let mut first_digit = 0;
    let mut last_digit = 0;
    for line in input.lines() {
        // for every character in line
        first_digit_flag = false;
        for c in line.chars() {
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let output = part1(input);
        assert_eq!(output, "142");
    }
}