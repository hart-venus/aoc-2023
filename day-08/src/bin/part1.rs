use std::collections::HashMap;


fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 


fn part1(input: &str) -> String {
    let mut split_input = input.split("\n\n").collect::<Vec<&str>>();
    if split_input.len() == 1 {
        // Windows line endings
        split_input = input.split("\r\n\r\n").collect::<Vec<&str>>();
    }

    let path_cycle = split_input[0].chars().collect::<Vec<char>>();
    // Routes is a hashmap of &str -> (&str, &str)
    let mut routes: HashMap<&str, (String, String)> = HashMap::new();
    for line in split_input[1].lines() {
        // set "AAA = (BBB, BBB)" to "AAA" -> ("BBB", "BBB")
        let mut split_line = line.split(" = (");
        let key = split_line.next().unwrap();
        let mut value = split_line.next().unwrap().to_string();
        value.pop();
        let mut split_value = value.split(", ");
        let value1 = split_value.next().unwrap();
        let value2 = split_value.next().unwrap();
        routes.insert(key, (value1.to_string(), value2.to_string()));
        
    }
    // print hashmap
    for (key, value) in &routes {
        println!("{}: ({}, {})", key, value.0, value.1);
    }

    let mut total_steps = 0;
    let mut initial_state = "AAA";
    let mut i = 0;
    loop {
        let mut next_state = "";
        if i >= path_cycle.len() {
            i = 0;
        }
        if path_cycle[i] == 'L' {
            next_state = &routes[initial_state].0;
        } else if path_cycle[i] == 'R' {
            next_state = &routes[initial_state].1;
        }
        println!("{} -> {}", initial_state, next_state);
        initial_state = next_state;
        total_steps += 1;
        i += 1;
        if (initial_state == "ZZZ") {
            break;
        }
    }


    total_steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let output = part1(input);
        assert_eq!(output, "6");
    }
}