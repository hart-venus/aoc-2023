use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 

// spread_directions should be a static array of tuples of (i, j) for each direction
const SPREAD_DIRECTIONS: [(i32, i32); 8] = [(0, 1), (1, 0), (-1, 0), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

fn part2(input: &str) -> String {
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

    let mut travelled: Vec<(i32, i32)> = vec![(curr_i, curr_j), roots[0], roots[1]];

    loop {
        let mut can_move = false;
        for root in roots.iter_mut(){
            can_move = false;
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
                        
                        *root = (new_i, new_j);
                        
                        
                        
                        
                        travelled.push(*root);
                        can_move = true;
                        break;
                    }
                }
            }
            if !can_move {
                break
            }
        }
        if !can_move {
            break
        }
    }
    // make an array of input with only the travelled points marked with their char
    // and the rest marked with '.'
    let mut output = input.clone();
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if travelled.contains(&(i as i32, j as i32)) {
                output[i][j] = c.clone();
            } else {
                output[i][j] = '.';
            }
        }
    }


    // raytracing even-odd algorithm
    // https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule

    let mut count = 0; 
    for row in output.iter_mut() {
        // remove all '-'s from the row
        row.retain(|&c| c != '-');
        let mut inside_flag = false;
        for (i, c) in row.iter().enumerate() {
            if *c == '.'{
                if inside_flag {
                    count += 1;
                }
                continue;
            }
            let next_char = row[i+1];
            if (*c == 'L' && next_char == '7')
            || (*c == 'F' && next_char == 'J')
            {
                continue; 
            }
            if (vec!['J', '|', '7', 'F', 'L', 'S'].contains(c)) {
                inside_flag = !inside_flag;
            }

        }
    }

    for row in output.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }

    count.to_string()
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
        let output = part2(input);
        assert_eq!(output, "1");
        // should be 1
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let output = part2(input);
        assert_eq!(output, "1");
        // should be 1
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let output = part2(input);
        assert_eq!(output, "4");
        // should be 4
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        let output = part2(input);
        assert_eq!(output, "4");
        // should be 4
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let output = part2(input);
        assert_eq!(output, "8");
        // should be 8
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let output = part2(input);
        assert_eq!(output, "10")
        // should be 10 
    }
}