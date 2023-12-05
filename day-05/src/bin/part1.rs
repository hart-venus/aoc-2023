fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 

fn part1(input: &str) -> String {
    // first, split the input by double newlines
    let mut input: Vec<&str> = input.split("\n\n").collect();
    // compatibility: use \r\n if the input is len 1
    if input.len() == 1{
        input = input[0].split("\r\n\r\n").collect();
    }
    println!("input len: {}", input.len());

    let mut seeds: Vec<i64> = input[0].split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    // from the second element of the input, iterate over each line group
    for line_group in input[1..].iter(){

        let line_nums_vec : Vec<Vec<i64>> = line_group.lines().skip(1)
            .map(|x| x.split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>())
            .collect::<Vec<Vec<i64>>>();

        // iterate over the seeds array to traduce each unit individually
        for seed in seeds.iter_mut(){
            for line_nums in line_nums_vec.iter(){
                let (dest, source, length) = (line_nums[0], line_nums[1], line_nums[2]);
                let seed_clone = *seed;
                if seed_clone >= source && seed_clone <= source + length - 1{
                    *seed = seed_clone + (dest - source);
                    break;
                }
            }
        }
    }
    // return the min value of the seeds array
    return seeds.iter().min().unwrap().to_string();
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = part1(input);
        assert_eq!(output, "35");
    }
}