fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 

fn part1(input: &str) -> String {
    let mut running_sum:u64 = 0;
    let line_len = input.lines().next().unwrap().len();
    
    // pad line with "." to make it easier to parse
    // one at the beginning and one at the end

    let mut padded_input = ".".repeat(line_len + 2);
    for line in input.lines() {
        padded_input.push_str(&format!("\n.{}.", line));
    }
    padded_input.push_str(&"\n");
    padded_input.push_str(&".".repeat(line_len + 2));

    let padded_lines = padded_input.lines().collect::<Vec<&str>>();
    let mut i: usize = 1;
    while i < padded_lines.len() - 1 {
        let mut j = 1;
        while j < line_len - 1 {
            
            let ch = padded_lines[i].chars().nth(j).unwrap();
            
            if !ch.is_digit(10){
                j += 1;
                continue;
            }

            let mut n: u64 = ch.to_digit(10).unwrap().into();
            j += 1;

            // while the j-th character is a digit, multiply n by 10 and add the char to n
            while padded_lines[i].chars().nth(j).unwrap().is_digit(10) {
                let ch = padded_lines[i].chars().nth(j).unwrap();
                let digit: u64 = ch.to_digit(10).unwrap().into();
                n = n * 10 + digit;
                j += 1;
            }
            // get the box around the number 
            let num_len = n.to_string().len();
            let bx = &padded_lines[i-1..i+2];
            // trim bx to just have the [j-num_len..j+1] chars
            let bx = bx.iter().map(|x| &x[j-num_len-1..j+1]).collect::<Vec<&str>>();
            // if there's anything in bx that's not a digit or a '.', add n to running_sum
            if bx.iter().any(|x| x.chars().any(|y| !y.is_digit(10) && y != '.')) {
                running_sum += n;
            }
        }
        i += 1
    }

    running_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = part1(input);
        assert_eq!(output, "4361");
    }
}