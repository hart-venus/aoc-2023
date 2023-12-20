use std::collections::HashMap;
#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32
}

#[derive(Debug)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Rule { // a < 2006
    key: String, 
    condition: Condition,
    value: i32,
    next: String,
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 

fn part1(input: &str) -> String {
    let mut running_sum = 0;
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut rules: Vec<&str> = Vec::new();
    let mut parts: Vec<&str> = Vec::new();
    let mut rules_flag = true;

    for line in lines {
        if line.is_empty() {
            rules_flag = false;
            continue;
        }
        if rules_flag {
            rules.push(line);
        } else {
            parts.push(line);
        }
    }
    
    let mut part_vec: Vec<Part> = Vec::new();
    // turns {x=2127,m=1623,a=2188,s=1013} into Part {x: 2127, m: 1623, a: 2188, s: 1013}
    for part in parts {
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        let mut key = String::new();
        let mut value = String::new();
        let mut key_flag = true;
        for c in part.chars() {
            if c == '{' || c == '}' {
                continue;
            }
            if c == '=' {
                key_flag = false;
                continue;
            }
            if c == ',' {
                match key.as_str() {
                    "x" => x = value.parse::<i32>().unwrap(),
                    "m" => m = value.parse::<i32>().unwrap(),
                    "a" => a = value.parse::<i32>().unwrap(),
                    "s" => s = value.parse::<i32>().unwrap(),
                    _ => panic!("Invalid key"),
                }
                key.clear();
                value.clear();
                key_flag = true;
                continue;
            }
            if key_flag {
                key.push(c);
            } else {
                value.push(c);
            }
        }
        match key.as_str() {
            "x" => x = value.parse::<i32>().unwrap(),
            "m" => m = value.parse::<i32>().unwrap(),
            "a" => a = value.parse::<i32>().unwrap(),
            "s" => s = value.parse::<i32>().unwrap(),
            _ => panic!("Invalid key"),
        }
        part_vec.push(Part {x, m, a, s});
    }

    let mut workflow_hash: HashMap<String, Vec<Rule>> = HashMap::new();
    // turns px{a<2006:qkq,m>2090:A,rfg} into rules for rule_hash["px"]
    for workflow in rules {
        let split_rules = workflow.split('{').collect::<Vec<&str>>();
        let hash_key = split_rules[0].to_string();
        let rules = split_rules[1].split(',').collect::<Vec<&str>>();
        let mut rule_vec: Vec<Rule> = Vec::new();
        for rule in rules {
            let split_rule = rule.split(':').collect::<Vec<&str>>();
            // next is the last element in split_rule without }
            let next = split_rule.last().unwrap().to_string().replace("}", "");
            
            // if split_rule[0] contains < 
            if split_rule[0].contains('<'){
                let rule_parts = split_rule[0].split('<').collect::<Vec<&str>>();
                let key = rule_parts[0].to_string();
                let value = rule_parts[1].parse::<i32>().unwrap();
                rule_vec.push(Rule {key, condition: Condition::LessThan, value, next});
            }
            else if split_rule[0].contains('>'){
                let rule_parts = split_rule[0].split('>').collect::<Vec<&str>>();
                let key = rule_parts[0].to_string();
                let value = rule_parts[1].parse::<i32>().unwrap();
                rule_vec.push(Rule {key, condition: Condition::GreaterThan, value, next});
            }
            else {
                let key = "a".to_string();
                let value = -1;
                rule_vec.push(Rule {key, condition: Condition::GreaterThan, value, next});
            }
            
        }
        workflow_hash.insert(hash_key, rule_vec);
    }

    for part in part_vec {
        let mut current_workflow = "in".to_string();

        loop {
            if current_workflow == "R" {
                break;
            }
            if current_workflow == "A" {
                running_sum += part.a + part.m + part.x + part.s;
                break;
            }

            println!("current_workflow: {}", current_workflow);
            let conditions = workflow_hash.get(&current_workflow).unwrap();
            for condition in conditions {
                let key = &condition.key;
                let value = &condition.value;
                let next = &condition.next;
                let condition = &condition.condition;

                match condition {
                    Condition::LessThan => {
                        if key == "x" && part.x < *value {
                            current_workflow = next.to_string();
                            break;
                        }
                        else if key == "m" && part.m < *value {
                            current_workflow = next.to_string();
                            break;
                        }
                        else if key == "a" && part.a < *value {
                            current_workflow = next.to_string();
                            break;
                        }
                        else if key == "s" && part.s < *value {
                            current_workflow = next.to_string();
                            break;
                        }
                    },
                    Condition::GreaterThan => {
                        if key == "x" && part.x > *value {
                            current_workflow = next.to_string();
                            break;
                        }
                        else if key == "m" && part.m > *value {
                            current_workflow = next.to_string();
                            break;
                        }
                        else if key == "a" && part.a > *value {
                            current_workflow = next.to_string();
                            break;
                        }
                        else if key == "s" && part.s > *value {
                            current_workflow = next.to_string();
                            break;
                        }
                    },
                }
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
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
        let output = part1(input);
        assert_eq!(output, "19114");
    }
}