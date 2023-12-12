
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 


fn part1(input: &str) -> String {
    let input = input.trim();
    let mut running_sum = 0;
    for line in input.lines() {
        let iter = line.split_whitespace();
        let original_string = iter.clone().next().unwrap();
        let num_arr = iter.skip(1).next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        // 2** number of ?s in the original string
        let num_iterations = 2usize.pow(original_string.chars().filter(|&c| c == '?').count() as u32);
        for i in 0..num_iterations {
            // replace all of the ?s for the corresponding bit, from right to left
            // e.g. if i is 000...1110, replace "??..#??" with ".#..###"
            let mut new_string = String::new();
            let mut i = i;
            
            for char in original_string.chars() {
                if char != '?' {
                    new_string.push(char);
                    continue;
                }
                let bit = i % 2;
                i /= 2;
                new_string.push_str(if bit == 0 { "." } else { "#" });
            }
            if num_arr == new_string.split(".").filter(|&x| x != "").map(|x| x.len()).collect::<Vec<_>>() {
                running_sum += 1;
            }
        }
    }
    running_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let output = part1(input);
        assert_eq!(output, "21");
    }
}