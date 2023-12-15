use std::collections::HashMap;


fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 


fn part2(input: &str) -> String {
    // hashmap of array of tuples (label, focal length)
    let mut boxes : HashMap<usize, Vec<(String, usize)>>;
    boxes = HashMap::new();
    let input = input.trim();
    let mut running_sum: i32 = 0;

    for slice in input.split(","){
        let mut hash_val = 0;
        
        let mut i = 0;
        loop {
            let c = slice.chars().nth(i).unwrap();
            if c == '-' || c == '=' {
                break;
            }
            hash_val += c as usize; // ASCII value
            hash_val *= 17;
            hash_val %= 256;
            i += 1;
        }

        let op = slice.chars().nth(i).unwrap();

        if op == '-' {
            if boxes.contains_key(&hash_val) {
                // remove any elements with the label matching the first chars
                boxes.get_mut(&hash_val).unwrap().retain(|x| x.0 != slice[0..i]);
            }
        }
        
        if op == '=' {
            let focal_length = slice[i+1..].parse::<usize>().unwrap();
            if boxes.contains_key(&hash_val) {
                // if the array already contains the label, update the focal length
                let mut found = false;
                for j in 0..boxes.get(&hash_val).unwrap().len() {
                    if boxes.get(&hash_val).unwrap()[j].0 == slice[0..i] {
                        boxes.get_mut(&hash_val).unwrap()[j].1 = focal_length;
                        found = true;
                        break;
                    }
                }
                // else, put the label and focal length in the array at the very end
                if !found {
                    boxes.get_mut(&hash_val).unwrap().push((slice[0..i].to_string(), focal_length));
                }

            } else {
                boxes.insert(hash_val, vec![(slice[0..i].to_string(), focal_length)]);
            }
        }
    }
    
    // print all of the boxes
    for (key, value) in &boxes {
        for (i, (_, focal_length)) in value.iter().enumerate() {

            running_sum += ((key + 1) * (i + 1) * focal_length) as i32;
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
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
        let output = part2(input);
        assert_eq!(output, "145");
    }
}