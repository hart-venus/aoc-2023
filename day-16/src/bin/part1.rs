
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn coords(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    fn symbol(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

struct Beam {
    x: i32,
    y: i32,
    direction: Direction
}

impl Beam {

    fn after(&self, character: char) -> Vec<Beam> {
        let mut beams: Vec<Beam> = Vec::new();

        match character {
            '.' => {
                let (dx, dy) = self.direction.coords();
                beams.push(Beam { x: self.x + dx, y: self.y + dy, direction: self.direction });
            },
            '/' => {
                let direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                let (dx, dy) = direction.coords();
                beams.push(Beam { x: self.x + dx, y: self.y + dy, direction: direction });
            }
            '\\' => {
                let direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                let (dx, dy) = direction.coords();
                beams.push(Beam { x: self.x + dx, y: self.y + dy, direction: direction });
            },
            '|' => {
                match self.direction {
                    Direction::Down | Direction::Up => {
                        let (dx, dy) = self.direction.coords();
                        beams.push(Beam { x: self.x + dx, y: self.y + dy, direction: self.direction });
                    },
                    Direction::Left | Direction::Right => {
                        let (dx1, dy1) = Direction::Up.coords();
                        let (dx2, dy2) = Direction::Down.coords();
                        beams.push(Beam { x: self.x + dx1, y: self.y + dy1, direction: Direction::Up });
                        beams.push(Beam { x: self.x + dx2, y: self.y + dy2, direction: Direction::Down });
                    },
                }
            },
            '-' => {
                match self.direction {
                    Direction::Left | Direction::Right => {
                        let (dx, dy) = self.direction.coords();
                        beams.push(Beam { x: self.x + dx, y: self.y + dy, direction: self.direction });
                    },
                    Direction::Down | Direction::Up => {
                        let (dx1, dy1) = Direction::Left.coords();
                        let (dx2, dy2) = Direction::Right.coords();
                        beams.push(Beam { x: self.x + dx1, y: self.y + dy1, direction: Direction::Left });
                        beams.push(Beam { x: self.x + dx2, y: self.y + dy2, direction: Direction::Right });
                    },
                }
            },
            _ => panic!("Unknown character: {}", character)
        }

        return beams;
    }
}

fn part1(input: &str) -> String {
    let input = input.trim();
    let mut running_sum: i32 = 0;

    let input_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // output matrix has the same dimensions as input matrix
    // but is mutable
    let mut output_matrix: Vec<Vec<char>> = input_matrix.clone();
    let mut beams: Vec<Beam> = Vec::new();

    // this initializes a 3D matrix, where each cell is a vector of chars
    let mut char_memo: Vec<Vec<Vec<char>>> = vec![vec![vec![]; input_matrix[0].len()]; input_matrix.len()];

    beams.push(Beam { x: 0, y: 0, direction: Direction::Right });

    loop {
        // pop the last beam from the beams vector
        if beams.len() == 0 {
            break;
        }

        let beam = beams.pop().unwrap();        
        // if x or y are out of bounds, continue to the next one
        if beam.x < 0 || beam.x >= input_matrix[0].len() as i32 || beam.y < 0 || beam.y >= input_matrix.len() as i32 {
            continue;
        }

        // if the direction's symbol is inside of this cell's memo, then we've already
        // calculated the next beams, so discard this beam and continue to the next one
        if char_memo[beam.y as usize][beam.x as usize].contains(&beam.direction.symbol()) {
            continue;
        }
        // put a # in the output matrix at the x, y coordinates
        output_matrix[beam.y as usize][beam.x as usize] = '#';
        // put the next beams in the beams vector at the beginning of the vector
        beams.splice(0..0, beam.after(input_matrix[beam.y as usize][beam.x as usize]));
        // put the direction's symbol in the memo
        char_memo[beam.y as usize][beam.x as usize].push(beam.direction.symbol());        
    }

    // return the count of # characters in the output matrix
    for line in output_matrix {
        for c in line {
            if c == '#' {
                running_sum += 1;
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
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";
        let output = part1(input);
        assert_eq!(output, "46");
    }
}