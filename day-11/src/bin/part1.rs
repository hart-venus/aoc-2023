
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 


fn part1(input: &str) -> String {
    let input = input.trim();

    let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut new_input: Vec<Vec<char>>;

    for _ in 0..2 {
        new_input = Vec::new();
        for line in input.iter() {

            // if all of the line is '.', insert a new line after this one
            if line.iter().all(|c| *c == '.') {
                new_input.push(line.clone());
            }
            new_input.push(line.clone());
        }
        input = new_input.clone();
        // transpose input matrix
        input = (0..input[0].len()).map(|i| input.iter().map(|row| row[i]).collect()).collect();
    }

    println!("New input \n{}", input.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));

    let mut galaxy_locations: Vec<(i32, i32)> = Vec::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '#' {
                galaxy_locations.push((y as i32, x as i32));
            }
        }
    }
    let mut running_sum = 0;
    for i in 0..galaxy_locations.len() {
        for j in i+1..galaxy_locations.len() {
            let dist_x = (galaxy_locations[i].0 - galaxy_locations[j].0).abs();
            let dist_y = (galaxy_locations[i].1 - galaxy_locations[j].1).abs();
            running_sum += dist_x + dist_y;
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
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let output = part1(input);
        assert_eq!(output, "374");
    }
}