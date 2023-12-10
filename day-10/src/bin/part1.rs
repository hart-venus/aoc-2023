use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 


fn part1(input: &str) -> String {
    // first, make input a Vec<Vec<char>> dividing by lines and chars
    let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    
    // add a border of '.' around the input
    let border = vec!['.'; input[0].len()];
    input.insert(0, border.clone());
    input.push(border.clone());
    for line in input.iter_mut() {
        line.insert(0, '.');
        line.push('.');
    }    


    // get the i and j of the starting point (S)
    let i = input.iter().position(|line| line.contains(&'S')).unwrap();
    let j = input[i].iter().position(|&c| c == 'S').unwrap();
    println!("i: {}, j: {}", i, j);

    let dir_hm = HashMap::from(
        [
            ("S".to_string(),vec![(0, 1), (1, 0), (-1, 0), (0, -1)]),
            ("|".to_string(),vec![(1, 0), (-1, 0)]),  // UD
            ("-".to_string(),vec![(0, 1), (0, -1)]),  // LR
            ("L".to_string(),vec![(0, 1), (-1, 0)]),  // UR
            ("J".to_string(),vec![(0, -1), (-1, 0)]), // UL
            ("F".to_string(),vec![(0, 1), (1, 0)]),   // DR
            ("7".to_string(),vec![(0, -1), (1, 0)]),  // DL
        ]
    );

    for row in input.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }

    let curr_i: i32 = i as i32;
    let curr_j: i32 = j as i32;
    let curr_dir = "S";
    let mut roots: Vec<(i32, i32)> = Vec::new();

    for possible_root in dir_hm.get(curr_dir).unwrap() {
        let new_i = curr_i + possible_root.0;
        let new_j = curr_j + possible_root.1;
        
        let new_root = input[new_i as usize][new_j as usize].to_string();
        if new_root != "." {
            let dirs: &Vec<(i32, i32)> = dir_hm.get(&new_root).unwrap();
            if dirs.contains(&(possible_root.0 * -1, possible_root.1 * -1)) {
                roots.push((new_i, new_j));
            }
        }
    }
    // assert roots.len() == 2;
    assert!(roots.len() == 2, "roots.len() == {}", roots.len());

    let mut steps = 1;
    let mut travelled: Vec<(i32, i32)> = vec![(curr_i, curr_j), roots[0], roots[1]];

    loop {
        steps += 1;
        for root in roots.iter_mut(){
            let mut can_move = false;
            let curr_dir = input[root.0 as usize][root.1 as usize].to_string();
            for possible_next in dir_hm.get(&curr_dir).unwrap() {
                let new_i = root.0 + possible_next.0;
                let new_j = root.1 + possible_next.1;
                if travelled.contains(&(new_i, new_j)) {
                    continue;
                }
                let new_root = input[new_i as usize][new_j as usize].to_string();
                if new_root != "." {
                    let dirs: &Vec<(i32, i32)> = dir_hm.get(&new_root).unwrap();
                    if dirs.contains(&(possible_next.0*-1, possible_next.1*-1)) {
                        println!("{:?} contains {:?}", dirs.contains(&(possible_next.0 * -1, possible_next.1 * -1)), (possible_next.0 * -1, possible_next.1 * -1));
                        println!("{} -> {} at {}, {}", input[root.0 as usize][root.1 as usize], new_root, new_i, new_j);
                        
                        *root = (new_i, new_j);
                        
                        
                        
                        
                        travelled.push(*root);
                        can_move = true;
                        break;
                    }
                }
            }
            if !can_move {
                return steps.to_string();
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let output = part1(input);
        assert_eq!(output, "4");
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let output = part1(input);
        assert_eq!(output, "8");
    }
}