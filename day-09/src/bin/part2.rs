fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 


fn part2(input: &str) -> String {
    let mut run_sum = 0;
    for line in input.lines() {
        let mut differences: Vec<Vec<i32>> = Vec::new();
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        differences.push(nums);
        // While the last row of differences is not all 0s
        while differences.last().unwrap().iter().any(|&x| x != 0) {
            let mut new_row: Vec<i32> = Vec::new();
            for i in 0..differences.last().unwrap().len() - 1 {
                new_row.push(differences.last().unwrap()[i + 1] - differences.last().unwrap()[i]);
            }
            differences.push(new_row);
        }
        // reverse the differences
        differences.reverse();
        let mut diffsum = 0;
        for i in 0..differences.len() {
            diffsum = -diffsum + differences[i][0];
        }
        run_sum += diffsum;
    }
    run_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let output = part2(input);
        assert_eq!(output, "2");
    }
}