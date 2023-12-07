fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 

fn part2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    // map each line's substring after : to an array of strings
    let times = lines[0].split(":").nth(1).unwrap().split_whitespace();
    let distances = lines[1].split(":").nth(1).unwrap().split_whitespace();

    // coalesce the strings into a single string then parse to int
    let time = times.collect::<String>().parse::<i64>().unwrap();
    let distance = distances.collect::<String>().parse::<i64>().unwrap();


    let mut ways_to_beat = 0;
    for i in 1..time.clone() {
        if ((i)*(time.clone()-i)) > distance.clone(){
            ways_to_beat += 1;
        }
    }

    ways_to_beat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = part2(input);
        assert_eq!(output, "71503");
    }
}