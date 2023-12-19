#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn area (points: Vec<Point>) -> f64{ 
    // uses the gauss area formula
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut sum:f64 = 0.0;
    for i in 0..points.len() {
        let j = (i+1) % points.len();
        sum += points[i].x * points[j].y - points[j].x * points[i].y;
    }
    sum.abs() / 2.0
}

fn rotate_right(x: f64, y: f64, theta: f64) -> (i32, i32) {
    // rotates the vector (x, y) theta radians clockwise
    let x_prime = x * theta.cos() + y * theta.sin();
    let y_prime = -x * theta.sin() + y * theta.cos();
    (x_prime as i32, y_prime as i32)
}

fn get_normalized_diff (p1: Point, p2: Point) -> (f64, f64) {
    // returns (-1, 0) if p1 is to the left of p2
    // returns (1, 0) if p1 is to the right of p2
    // returns (0, -1) if p1 is above p2
    // returns (0, 1) if p1 is below p2
    let diff_x = p1.x - p2.x;
    let diff_y = p1.y - p2.y;
    let magnitude = (diff_x.powi(2) + diff_y.powi(2)).sqrt();
    (diff_x / magnitude, diff_y / magnitude)
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 

fn part2(input: &str) -> String {
    let input = input.trim();
    let mut points: Vec<Point> = Vec::new();
    let mut curr_x = 0;
    let mut curr_y = 0;

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let color = words[2];

        // match the second to last character of the color to get the direction
        let dir:char = color.chars().nth(color.len()-2).unwrap();
        // the 2nd to fifth characters of the color are the distance in hex (lowercase)
        println!("{}", &color[2..7]);
        let dist = i32::from_str_radix(&color[2..7], 16).unwrap();    


        match dir {
            '0' => curr_x += dist,
            '2' => curr_x -= dist,
            '3' => curr_y -= dist,
            '1' => curr_y += dist,
            _ => panic!("unknown direction"),
        }
        
        points.push(Point{x: curr_x as f64, y: curr_y as f64});    
    }

    let mut points_offset = points.clone();
    for i in 0..points.len() {
        let next_point = points[(i+1) % points.len()];
        let point = points[i];
        let (x, y) = get_normalized_diff(point, next_point);
        // rotate it 90 degrees clockwise
        let (x, y) = rotate_right(-x, -y, std::f64::consts::FRAC_PI_2);
        // get offset point i and i+1 by 0.5 in the direction of the vector
        let mut offset_point = points_offset[i];
        offset_point.x += x as f64 * 0.5;
        offset_point.y += y as f64 * 0.5;
        points_offset[i] = offset_point;

        let mut offset_point = points_offset[(i+1) % points.len()];
        offset_point.x += x as f64 * 0.5;
        offset_point.y += y as f64 * 0.5;
        points_offset[(i+1) % points.len()] = offset_point;

    }
    
    area(points_offset).to_string()
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
        let output = part2(input);
        assert_eq!(output, "952408144115");
    }
}