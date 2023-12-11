
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 


fn part2(input: &str) -> String {
    let input = input.trim();

    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxy_locations: Vec<(i64, i64)> = Vec::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '#' {
                galaxy_locations.push((y as i64, x as i64));
            }
        }
    }
    // print input
    let mut running_sum = 0;
    let factor = 1000000;
    for i in 0..galaxy_locations.len() {
        for j in i+1..galaxy_locations.len() {

            let mut dist_y = (galaxy_locations[i].0 - galaxy_locations[j].0).abs();

            // get all rows between i and j
            let rows: Vec<Vec<char>> = input[galaxy_locations[i].0 as usize..galaxy_locations[j].0 as usize].to_vec();
            // get count of rows which are only '.'
            let mut count = 0;
            for row in rows {
                if row.iter().all(|c| *c == '.') {
                    count += 1;
                }
            }         

            dist_y = dist_y - count + (count * factor);

            let mut dist_x = (galaxy_locations[i].1 - galaxy_locations[j].1).abs();

            // edge case: the first galaxy can be right of the second galaxy
            // in which case we need to swap the x values
            let mut first_galaxy = galaxy_locations[i].1;
            let mut second_galaxy = galaxy_locations[j].1;

            if first_galaxy > second_galaxy {
                std::mem::swap(&mut first_galaxy, &mut second_galaxy);
            }

            // get columns between first and second galaxy
            let columns: Vec<Vec<char>> = input.iter().map(|row| row[first_galaxy as usize..second_galaxy as usize].to_vec()).collect();

            // get count of indexes which are only '.'
            let mut count = 0;
            for i in 0..columns[0].len() {
                if columns.iter().all(|row| row[i] == '.') {
                    count += 1;
                }
            }

            dist_x = dist_x - count + (count * factor);

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
        let output = part2(input);
        assert_eq!(output, "8410");
    }
}