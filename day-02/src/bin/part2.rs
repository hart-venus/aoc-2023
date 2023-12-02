
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 

fn part2(input: &str) -> String {
    let mut running_sum:u64 = 0;

    for line in input.lines() {
        let mut min_cubes = std::collections::HashMap::new();
        min_cubes.insert("red", 0);
        min_cubes.insert("green", 0);
        min_cubes.insert("blue", 0);

        // Get line slice after ":"
        let line = line.split(":").collect::<Vec<&str>>()[1];
        // Get semicolon separated list of cube_sets
        let cube_sets = line.split(";").collect::<Vec<&str>>();
        for cube_set in cube_sets {
            let colors = cube_set.split(",").collect::<Vec<&str>>();
            // create a dict of color: number of cubes ("3 blue" -> "blue": 3)
            let mut color_dict = std::collections::HashMap::new();
            for color in colors {
                let color = color.trim();
                let color = color.split(" ").collect::<Vec<&str>>();
                color_dict.insert(color[1], color[0]);
            }
            // if any entry in color_dict is greater than in min_cubes
            // set min_cubes to the new value
            for (color, count) in color_dict {
                if min_cubes.get(color).unwrap() < &count.parse::<u64>().unwrap() {
                    min_cubes.insert(color, count.parse::<u64>().unwrap());
                }
            }
        } 
        // add the power of min_cubes (red*green*blue) to running_sum
        running_sum += min_cubes.get("red").unwrap() * min_cubes.get("green").unwrap() * min_cubes.get("blue").unwrap();
    }
    running_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part2(input);
        assert_eq!(output, "2286");
    }
}