
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 

fn part1(input: &str) -> String {
    let input = input.trim();


    let arr_dim = 2000;
    // make a 100x100 matrix of '.'
    let mut matrix = vec![vec!['.'; arr_dim]; arr_dim];

    let mut curr_i = arr_dim/2;
    let mut curr_j = arr_dim/2;
    matrix[curr_i][curr_j] = '#';

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let dir = words[0];
        let dist: i32 = words[1].parse().unwrap();

        for _ in 0..dist {
            match dir {
                "R" => curr_j += 1,
                "L" => curr_j -= 1,
                "U" => curr_i -= 1,
                "D" => curr_i += 1,
                _ => panic!("unknown direction"),
            }
            matrix[curr_i][curr_j] = '#';
            
        }

    }

    let mut w_count = 0;
    // now we use flood fill to fill in the rest of the matrix with #'s
    let mut queue = vec![(0, 0)];
    while !queue.is_empty() {
        let (i, j) = queue.pop().unwrap();
        if i > 0 && matrix[i-1][j] == '.' {
            matrix[i-1][j] = 'w';
            w_count += 1;
            queue.push((i-1, j));
        }
        if i < arr_dim-1 && matrix[i+1][j] == '.' {
            matrix[i+1][j] = 'w';
            w_count += 1;
            queue.push((i+1, j));
        }
        if j > 0 && matrix[i][j-1] == '.' {
            matrix[i][j-1] = 'w';
            w_count += 1;
            queue.push((i, j-1));
        }
        if j < arr_dim-1 && matrix[i][j+1] == '.' {
            matrix[i][j+1] = 'w';
            w_count += 1;
            queue.push((i, j+1));
        }
    }    

    (arr_dim.pow(2) - w_count).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
        let output = part1(input);
        assert_eq!(output, "62");
    }
}