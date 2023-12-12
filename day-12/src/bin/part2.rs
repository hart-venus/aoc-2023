use std::collections::HashMap;


fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 

// Part 2 inspired by https://github.com/AlbertVeli/AdventOfCode/blob/master/2023/12/12_2.py

struct Memoizer {
    // define a data structure with a cache to be able to pass to memoized function
    cache: HashMap<(String, Vec<usize>, usize), usize>,
}

impl Memoizer {
    // Constructor instances a new cache
    fn new() -> Memoizer {
        Memoizer {
            cache: HashMap::new(),
        }
    }

    // Get function returns the value of the memoized function if it exists in the cache
    // If not, it calls the memoized function and stores the result in the cache
    fn get(&mut self, string: &str, groups:Vec<usize>, group_size: usize) -> usize {
        let key = (string.to_string(), groups.clone(), group_size);


        if let Some(&value) = self.cache.get(&key) {
            return value;
        }

        let result = self.n_arrangements(string, groups, group_size);
        self.cache.insert(key, result);
        result
    }

    // Count the number of possible arrangements in a monogram-like string
    // string: "???.###" where ? is a wildcard and . or # is a fixed character
    // groups: 1, 1, 3 where each number is the length of a group of fixed #s
    // split up by .s
    // group_size: the length of the current group of fixed #s in a row

    fn n_arrangements(&mut self, string: &str, groups: Vec<usize>, group_size: usize) -> usize {
        // base case -> string is null

        // if I don't have any more characters left
        if string.len() == 0 {
            // If my last group matches the last group in the list of groups
            if groups.len() == 0 && group_size == 0 {
                return 1;
            }
            if groups.len() == 1 && group_size == groups[0] {
                return 1;
            }
            return 0;
        }
        
        // edge cases in case I tried to make a longer or shorter group than the first group 
        // (the current one is the first one i'm trying to match) 

        if groups.len() > 0 && group_size > groups[0] {
            return 0;
        }

        if groups.len() == 0 && group_size > 0 {
            return 0;
        }

        let mut total_count = 0; // total possibilities under current string
        let curr_spring = string.chars().next().unwrap(); // current character


        // if the current character is a # I add it to the current group and "eat" it
        if curr_spring == '#' || curr_spring == '?' {
            total_count += self.get(&string[1..], groups.clone(), group_size + 1);
        }
        // if the current character is a . I check if the current group matches the first group
        // in the list of groups. If it does, I "eat" the . and the group and move on to the next
        // character. If it doesn't, I "eat" the . and move on to the next character.
        if curr_spring == '.' || curr_spring == '?' {
            if groups.len() > 0 && group_size == groups[0] {
                total_count += self.get(&string[1..], groups[1..].to_vec(), 0);
            }
            else if group_size == 0 {
                total_count += self.get(&string[1..], groups.clone(), 0);
            }
        }

        total_count
    }
}




fn part2(input: &str) -> String {
    let input = input.trim();
    let mut running_sum = 0;
    // parameters of memoized function are (String, Vec<usize>, usize)
    // result of memoized function is usize
    let mut memoizer = Memoizer::new();    


    for line in input.lines() {
        let iter = line.split_whitespace();
        let mut my_string: String = iter.clone().next().unwrap().to_string();
        let mut num_arr = iter.skip(1).next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        // append 4 copies of num_arr to itself
        let original_num_arr = num_arr.clone();
        let original_string = "?".to_string() + my_string.as_str();
        for _ in 0..4 {
            num_arr.extend(original_num_arr.clone());
            my_string.push_str(&original_string);
        }

        running_sum += memoizer.get(my_string.as_str(), num_arr, 0);

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
        let output = part2(input);
        assert_eq!(output, "525152");
    }
}