
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
} 


fn part1(input: &str) -> String {
    let input = input.trim();
    let mut running_sum: i32 = 0;

    for slice in input.split(","){
        let mut hash_val = 0;
        
        for c in slice.chars(){
            hash_val += c as i32; // ASCII value
            hash_val *= 17;
            hash_val %= 256;
        }

        running_sum += hash_val;
    }



    running_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
        let output = part1(input);
        assert_eq!(output, "1320");
    }
}