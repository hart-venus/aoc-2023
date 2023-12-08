use std::collections::HashMap;


fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 

fn lcm(nums: &[usize]) -> usize {
    let mut nums = nums.to_vec();
    nums.sort();
    let mut lcm = nums[0];
    for num in nums.iter().skip(1) {
        lcm = lcm * num / gcd(lcm, *num);
    }
    lcm
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}



fn part2(input: &str) -> String {
    let mut split_input = input.split("\n\n").collect::<Vec<&str>>();
    if split_input.len() == 1 {
        // Windows line endings
        split_input = input.split("\r\n\r\n").collect::<Vec<&str>>();
    }

    let path_cycle = split_input[0].chars().collect::<Vec<char>>();
    // Routes is a hashmap of &str -> (&str, &str)
    let mut routes: HashMap<String, (String, String)> = HashMap::new();
    let mut ghost_paths: Vec<String> = Vec::new();
    for line in split_input[1].lines() {
        // set "AAA = (BBB, BBB)" to "AAA" -> ("BBB", "BBB")
        let mut split_line = line.split(" = (");
        let key = split_line.next().unwrap().to_string();
        // if key ends with A, append to ghost_paths
        if key.ends_with("A") {
            ghost_paths.push(key.to_string());
        }
        let mut value = split_line.next().unwrap().to_string();
        value.pop();
        let mut split_value = value.split(", ");
        let value1 = split_value.next().unwrap();
        let value2 = split_value.next().unwrap();
        routes.insert(key, (value1.to_string(), value2.to_string()));
        
    }

    let mut ghost_steps: Vec<usize> = Vec::new();

    
    for ghost_path in ghost_paths.iter_mut() {
        let mut i = 0;
        let mut step_count = 0;
        loop {
            if i >= path_cycle.len() {
                i = 0;
            }

            let mut next_state = "";
            if path_cycle[i] == 'L' {
                next_state = &routes[&ghost_path.to_string()].0;
            } else if path_cycle[i] == 'R' {
                next_state = &routes[&ghost_path.to_string()].1;
            }

            // if next_state ends with z, break
            if next_state.ends_with("Z") {
                ghost_steps.push(step_count+1);
                break;
            }

            *ghost_path = next_state.to_string();

            i += 1;
            step_count += 1;
        }
    }

    // return lcm of ghost_steps
    lcm(&ghost_steps).to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let output = part2(input);
        assert_eq!(output, "6");
    }
}