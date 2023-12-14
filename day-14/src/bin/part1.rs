
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 


fn part1(input: &str) -> String {
    let input = input.trim();
    let mut running_sum: i32 = 0;
    // for column in input 
    for i in 0..input.lines().nth(0).unwrap().len() {
        // for row in column
        let mut offset = 0;
        for j in 0..input.lines().count() {
            let char = input.lines().nth(j).unwrap().chars().nth(i).unwrap();
            if char == '#' {
                offset = j + 1;
            }
            else if char == 'O' {
                running_sum += ( input.lines().count() - offset) as i32;
                offset += 1;

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
        let output = part1(input);
        assert_eq!(output, "136");
    }
}