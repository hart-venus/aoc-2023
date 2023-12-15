use std::cmp;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 


fn part2(input: &str) -> String {
    let input = input.trim();
    let mut running_sum: i32 = 0;
    let mut index_offset: usize = 0;

    // turn input into a matrix
    let mut matrix = vec![];
    let mut matrix_mem: Vec<Vec<Vec<char>>> = vec![];
    let mut score_mem: Vec<usize> = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for char in line.chars() {
            row.push(char);
        }
        matrix.push(row);
    }
    loop {
        let mut curr_score = 0;
        // first part of the cycle (go north)
        // for column in input 
        for i in 0..matrix[0].len() {
            // for row in column
            let mut offset = 0;
            for j in 0..matrix.len() {
                let char: char = matrix[j][i];
                if char == '#' {
                    offset = j + 1;
                }
                else if char == 'O' {
                    // set matrix[offset][i] to 'O'
                    matrix[j][i] = '.';
                    matrix[offset][i] = 'O';
                    offset += 1;
                }

            }
        }

        // second part of the cycle (go west)
        // for row in input
        for i in 0..matrix.len() {
            // for column in row
            let mut offset = 0;
            for j in 0..matrix[0].len() {
                let char = matrix[i][j];
                if char == '#' {
                    offset = j + 1;
                }
                else if char == 'O' {
                    // set matrix[i][offset] to 'O'
                    matrix[i][j] = '.';
                    matrix[i][offset] = 'O';
                    offset += 1;
                }
            }
        }

        // third part of the cycle (go south)
        // for column in input
        for i in 0..matrix[0].len() {
            // for row in column
            let mut offset: usize = matrix.len() - 1;
            for j in (0..matrix.len()).rev() {
                let char = matrix[j][i];
                if char == '#' {
                    offset = cmp::max((j as i32) - 1 as i32, 0 as i32) as usize;
                }
                else if char == 'O' {
                    curr_score += ( input.lines().count() - (offset as usize)) as i32;
                    // set matrix[offset][i] to 'O'
                    matrix[j][i] = '.';
                    matrix[offset as usize][i] = 'O';
                    offset = cmp::max((offset as i32 - 1), 0 as i32) as usize;
                }
            }
        }

        // fourth part of the cycle (go east)
        // for row in input
        for i in 0..matrix.len() {
            // for column in row
            let mut offset: usize = matrix[0].len() - 1;
            for j in (0..matrix[0].len()).rev() {
                let char = matrix[i][j];
                if char == '#' {
                    offset = cmp::max((j as i32) - 1 as i32, 0 as i32) as usize;
                }
                else if char == 'O' {
                    // set matrix[i][offset] to 'O'
                    matrix[i][j] = '.';
                    matrix[i][offset as usize] = 'O';
                    offset = cmp::max((offset as i32 - 1) as i32, 0 as i32) as usize;
                }
            }
        }

        // if my matrix_mem contains matrix, print the index of the matrix in matrix_mem and break
        if matrix_mem.contains(&matrix) {
            println!("matrix_mem contains matrix");
            index_offset = matrix_mem.iter().position(|x| *x == matrix).unwrap();
            println!("index of matrix in matrix_mem: {}", matrix_mem.iter().position(|x| *x == matrix).unwrap());
            break;
        }
        
        matrix_mem.push(matrix.clone());
        score_mem.push(curr_score as usize);
    }

    println!("score_mem: {:?}", score_mem);
    // grab values from score_mem[index_offset..]
    let mut scores: Vec<usize> = vec![];
    for i in index_offset..score_mem.len() {
        scores.push(score_mem[i]);
    }
    println!("scores: {:?}", scores);
    let res = scores[(1_000_000_000 - index_offset - 1) % scores.len()];


    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let output = part2(input);
        assert_eq!(output, "64");
    }
}