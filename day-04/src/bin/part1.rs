fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 

fn part1(input: &str) -> String {
    let mut running_sum = 0;
    for line in input.lines() {
        let contents: &str = line.split(":").collect::<Vec<&str>>()[1].trim();
        let winning_numbers = contents.split(" | ").collect::<Vec<&str>>()[0].split_whitespace().collect::<Vec<&str>>();
        let my_numbers = contents.split(" | ").collect::<Vec<&str>>()[1].split_whitespace().collect::<Vec<&str>>();

        // count how many numbers in my_numbers are in winning_numbers
        let mut count = 0;
        for my_number in my_numbers {
            if winning_numbers.contains(&my_number) {
                count += 1;
            }
        }
        if count > 0{
            running_sum += (2 as u32).pow(count - 1);
        }
    }
    running_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = part1(input);
        assert_eq!(output, "13");
    }
}