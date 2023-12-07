use std::cmp::Ordering;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
} 

struct HandString(String);


impl Ord for HandString {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare the first character of each string
        let order = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
        let self_char = self.0.chars().nth(0).unwrap();
        let other_char = other.0.chars().nth(0).unwrap();
        let self_index = order.iter().position(|&x| x == self_char).unwrap();
        let other_index = order.iter().position(|&x| x == other_char).unwrap();
        let order = self_index.cmp(&other_index);
        // if they are the same, recursively call cmp on the next substring as a HandString
        if order == Ordering::Equal {
            let self_substring = self.0.chars().skip(1).collect::<String>();
            let other_substring = other.0.chars().skip(1).collect::<String>();
            let self_hand = HandString(self_substring);
            let other_hand = HandString(other_substring);
            return self_hand.cmp(&other_hand);
        }
        else {
            return order;
        }
    }
}

impl PartialOrd for HandString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for HandString {}


fn part2(input: &str) -> String {
    let mut hands = input.split("\n").collect::<Vec<_>>();
    if hands.len()==0 {
        hands = input.split("\r\n").collect::<Vec<_>>();
    }

    let mut hand_groups : Vec<Vec<&str>> = Vec::new();
    // insert 7 hand groups
    for _ in 0..7 {
        hand_groups.push(Vec::new());
    }

    for hand in hands {
        let cards = hand.split(" ").collect::<Vec<_>>()[0];
        // make hashmap of 'char' => count of char
        let mut card_map = std::collections::HashMap::new();
        for c in cards.chars() {
            if c == 'J' {continue;}
            let count = card_map.entry(c).or_insert(0);
            *count += 1;
        }

        let most_common_card: &char;

        // if there are no entries, hand is JJJJJ
        if card_map.len() == 0 {
            hand_groups[0].push(hand);
            continue;
        }

        most_common_card = card_map.iter().max_by_key(|&(_, count)| count).unwrap().0;

        // replace all Js with the most common card
        let new_cards = cards.replace("J", &most_common_card.to_string());
        // make hashmap of 'char' => count of char
        let mut card_map = std::collections::HashMap::new();
        for c in new_cards.chars() {
            let count = card_map.entry(c).or_insert(0);
            *count += 1;
        }


        // if there is only one entry, then it is a group of 5
        // add it to hand_groups[0]

        if card_map.len() == 1 {hand_groups[0].push(hand)}
        // if there are two entries and one of them is 4, then it is 4OAK
        else if card_map.len() == 2 && card_map.values().any(|&x| x == 4) {hand_groups[1].push(hand)}
        // else if there are only two entries it's a FH
        else if card_map.len() == 2 {hand_groups[2].push(hand)}
        // else if there are three entries and one of them is 3, then it is 3OAK
        else if card_map.len() == 3 && card_map.values().any(|&x| x == 3) {hand_groups[3].push(hand)}
        // else if there are three entries it's a 2P
        else if card_map.len() == 3 {hand_groups[4].push(hand)}
        // else if there are four entries it's a 1P
        else if card_map.len() == 4 {hand_groups[5].push(hand)}
        // else it's a high card
        else {hand_groups[6].push(hand)}
    
    }

    // finally, let's sort the hand groups by turning them into handstrings
    let mut hand_strings = Vec::new();
    for hand_group in hand_groups {
        let mut hand_strings_group = Vec::new();
        for hand in hand_group {
            hand_strings_group.push(HandString(hand.to_string()));
        }
        // print the hand strings
        println!("{:?}", hand_strings_group.iter().map(|x| x.0.clone()).collect::<Vec<_>>());
        hand_strings.push(hand_strings_group);
    }

    // sort each hand group
    for hand_strings_group in hand_strings.iter_mut() {
        hand_strings_group.sort();
    }

    // fuse all of the hand groups together sequentially
    let hand_strings = hand_strings.into_iter().flatten().collect::<Vec<_>>();
    let mut running_sum = 0;

    for (i, hand_string) in hand_strings.iter().enumerate() {
        let bid = hand_string.0.split(" ").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        running_sum += (bid as usize) * (hand_strings.len() - (i));
    }
    
    running_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let output = part2(input);
        assert_eq!(output, "5905");
    }
}