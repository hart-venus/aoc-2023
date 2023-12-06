use std::cmp::{max, min};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 

fn part2(input: &str) -> String {
    // first, split the input by double newlines
    let mut input: Vec<&str> = input.split("\n\n").collect();
    // compatibility: use \r\n if the input is len 1
    if input.len() == 1{
        input = input[0].split("\r\n\r\n").collect();
    }

    // seeds are now gonna be an array of [start, len] pairs
    // space separated numbers in input start1 len1 start2 len2 ...
    let seeds_raw = input[0].split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut seeds: Vec<Vec<i64>> = Vec::new();
    for i in 0..seeds_raw.len()/2{
        seeds.push(vec![seeds_raw[2*i], seeds_raw[2*i+1]]);
    }

    println!("seeds: {:?}", seeds);

    // from the second element of the input, iterate over each line group
    for line_group in input[1..].iter(){
        let mut new_seeds: Vec<Vec<i64>> = Vec::new();

        let line_nums_vec : Vec<Vec<i64>> = line_group.lines().skip(1)
            .map(|x| x.split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>())
            .collect::<Vec<Vec<i64>>>();

        // iterate over the seeds array to traduce each unit individually
        let mut i = 0;
        while i < seeds.len(){
            let seed = &seeds[i];
            let (seed_from, seed_len) = (seed[0], seed[1]);
            let mut seed_chosen = false;
            for line_nums in line_nums_vec.iter(){
                let (destination, source, length) = (line_nums[0], line_nums[1], line_nums[2]);
                
                if (seed_from >= source && seed_from <= source+length-1) || (seed_from + seed_len - 1 >= source && seed_from + seed_len - 1 <= source+length-1) || (seed_from <= source && seed_from + seed_len - 1 >= source+length-1){
                    let left_half = vec![seed_from, max(0, source-seed_from)];
                    let right_half = vec![min(seed_from+seed_len, source+length), max(0, seed_from+seed_len-source-length)];
                    let mut center = vec![max(seed_from, source), min(seed_from+seed_len, source+length)-max(seed_from, source)];
                    if left_half[1] > 0{
                        seeds.push(left_half);
                    }
                    if right_half[1] > 0{
                        seeds.push(right_half);
                    }
                    // process center by adding the difference between source and dest to the first element
                    center[0] += destination - source;
                    new_seeds.push(center);
                    seed_chosen = true;
                }
            
            }
            if !seed_chosen {
                new_seeds.push(vec![seed_from, seed_len]);
            }
            i += 1;
        }
        seeds = new_seeds;
    }
    return seeds.iter().map(|x| x[0]).min().unwrap().to_string();
    
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
        let output = part2(input);
        assert_eq!(output, "46");
    }
}