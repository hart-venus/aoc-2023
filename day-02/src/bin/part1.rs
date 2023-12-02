fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input, 12, 13, 14);
    dbg!(output);
} 

fn part1(input: &str, r:u64, g:u64, b:u64) -> String {
    let mut running_sum:u64 = 0;
    let mut game_id = 1;
    let mut color_hash = std::collections::HashMap::new();
    color_hash.insert("red", r);
    color_hash.insert("green", g);
    color_hash.insert("blue", b);

    println!("r: {}, g: {}, b: {}", r, g, b);
    for line in input.lines() {
        // Get line slice after ":"
        let line = line.split(":").collect::<Vec<&str>>()[1];
        // Get semicolon separated list of cube_sets
        let cube_sets = line.split(";").collect::<Vec<&str>>();
        let mut valid = true;
        for cube_set in cube_sets {
            let colors = cube_set.split(",").collect::<Vec<&str>>();
            // create a dict of color: number of cubes ("3 blue" -> "blue": 3)
            let mut color_dict = std::collections::HashMap::new();
            for color in colors {
                let color = color.trim();
                let color = color.split(" ").collect::<Vec<&str>>();
                color_dict.insert(color[1], color[0]);
            }
            // if any entry in color_dict is greater than in color_hash
            // then this game is invalid
            for (color, count) in color_dict {
                if color_hash.get(color).unwrap() < &count.parse::<u64>().unwrap() {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        } 
        if valid {
            running_sum += game_id;
        }

        // finally, increase the game id. 
        game_id += 1;
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
        let output = part1(input, 12, 13, 14);
        assert_eq!(output, "8");
    }
}