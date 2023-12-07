fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 

fn part1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    // map each line's substring after : to an array of ints
    let times = lines[0].split(":").nth(1).unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let distances = lines[1].split(":").nth(1).unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    // zip each time and distance 
    let races: Vec<(&i32, &i32)> = times.iter().zip(distances.iter()).collect::<Vec<_>>();
    let mut margin = 1;
    for race in races.iter(){
        let mut ways_to_beat = 0;
        let (time, distance) = race.clone();
        for i in 1..time.clone() {
            if ((i)*(time.clone()-i)) > distance.clone(){
                ways_to_beat += 1;
            }
        }
        margin *= ways_to_beat;
    }

    margin.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = part1(input);
        assert_eq!(output, "288");
    }
}