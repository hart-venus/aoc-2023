
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 


fn part2(input: &str) -> String {
    let input = input.trim();
    let mut running_sum = 0;

    let double_line_ending;

    if input.contains("\r") {
        double_line_ending = "\r\n\r\n";
    } else {
        double_line_ending = "\n\n";
    }

    let mut input_matrixes: Vec<Vec<Vec<char>>> = input.split(double_line_ending).map(|x| {
        x.lines().map(|y| {
            y.chars().collect()
        }).collect()
    }).collect();   


    for matrix in input_matrixes.iter_mut() {
        // let's check if this matrix mirrors horizontally 
        let mut mirrored_index = -1;
        let mut transposed_flag = false;

        loop {
            for index in 1..matrix.len() {
                // get first index rows
                println!("checking index {}", index);
                let mut first_rows: Vec<Vec<char>> = (&matrix[0..index]).to_vec();
                first_rows.reverse();
                let mut mirrored = true;
                let mut i = index; 
                let mut errFlag = false;

                for row in first_rows.into_iter() {


                    if i >= matrix.len() {
                        break;
                    }

                    println!("comparing {:?} to {:?}", row, matrix[i]);

                    for j in 0..row.len() {
                        if row[j] != matrix[i][j] {
                            if errFlag {
                                mirrored = false;
                                break;
                            }
                            errFlag = true;
                        }
                    }

                    i += 1
                }
                if mirrored && errFlag {
                    mirrored_index = index as i32;
                    break;
                }
            }
            if mirrored_index != -1 {

                if transposed_flag {
                    running_sum += mirrored_index * 1;
                } else {
                    running_sum += mirrored_index * 100;
                }

                break;
            }
            
            let mut transposed_matrix: Vec<Vec<char>> = vec![];

            // transpose
            for i in 0..matrix[0].len() {
                transposed_matrix.push(vec![]);
                for j in 0..matrix.len() {
                    transposed_matrix[i].push(matrix[j][i]);
                }
            }
            
            // clone transposed matrix
            *matrix = transposed_matrix.clone();
            transposed_flag = true;
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let output = part2(input);
        assert_eq!(output, "400");
    }
}