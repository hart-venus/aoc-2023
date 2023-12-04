fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 

fn part2(input: &str) -> String {
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
            
            if !(ch=='*'){
                j += 1;
                continue;
            }
            j += 1;

            // get the 3 row box around the *
            let bx = &padded_lines[i-1..i+2];
            // get a new box with just the [j-1..j+2] chars
            let bx2 = bx.iter().map(|x| &x[j-2..j+1]).collect::<Vec<&str>>();   
            // get the count of how many numbers are in bx2
            // i.e. ["123"], ["1*3"], ["..."] -> 3 (123, 1, 3)
            let mut num_count = 0;
            // num indices are a 2d 2x2 array of integers
            let mut num_indices = [[0; 2]; 2];
            
            for (i, line) in bx2.iter().enumerate() {
                let mut num_flag = false;
                for (j2, ch) in line.chars().enumerate() {
                    if ch.is_digit(10) && (!num_flag) {
                        if num_count == 2 {
                            num_count += 1;
                            break;
                        }
                        num_indices[num_count] = [i, j2+j-1]; // index of asterisk + index of number inside window
                        num_count += 1;
                        num_flag = true;
                        
                    } 
                    else if (!ch.is_digit(10)) && num_flag {
                        num_flag = false;
                    }
                }
                if num_count > 2 {
                    break;
                }
            }            

            if num_count != 2 {
                j += 1;
                continue;
            }

            // now we know that there's exactly two numbers around the asterisk
            // and we know the location of one digit per number
            // now let's iterate over the three original rows and see if there's a number in the same column
            let mut curr_index = 0;
            let mut first_number : u64 = 0;
            let mut second_number : u64 = 0;
            
            for (i2, line) in bx.iter().enumerate() {
                let mut num_flag = false;
                let mut should_include = false;
                let mut curr_number: u64= 0;
                for (j2, ch) in line.chars().enumerate() {

                    if ch.is_digit(10){
                        let chn: u64 = ch.to_digit(10).unwrap().into();
                        curr_number = curr_number * 10 + chn;
                        num_flag = true;
                        if num_indices[curr_index][0] == i2 && num_indices[curr_index][1] == j2+1 {
                            should_include = true;
                        }
                    }
                    else if num_flag {
                        num_flag = false;
                        if should_include {
                            if curr_index == 0 {
                                first_number = curr_number;
                                curr_index += 1;
                            }
                            else {
                                second_number = curr_number;
                            }
                            should_include = false;
                        }
                        curr_number = 0;
                    }

                }
            }
            
            running_sum += first_number * second_number;
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
        let output = part2(input);
        assert_eq!(output, "467835");
    }
}