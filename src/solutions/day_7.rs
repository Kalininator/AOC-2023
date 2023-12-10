use sscanf::scanf;

pub struct Day7;
impl crate::Solution for Day7 {
    fn part1(&self, input: &str) {
        println!("{}", part1(input));
    }
    fn part2(&self, input: &str) {
        println!("{}", part2(input));
    }
}

fn part1(input: &str) -> usize {
    solve(input)
}

fn part2(input: &str) -> usize {
    solve(&input.replace('J', "*"))
}

fn solve(input: &str) -> usize {
    let mut entries = input
        .lines()
        .map(|l| scanf!(l, "{}", Entry).unwrap())
        .collect::<Vec<Entry>>();

    entries.sort_by(|a, b| {
        let a_hand = a.hand.chars().collect::<Vec<char>>();
        let b_hand = b.hand.chars().collect::<Vec<char>>();
        let a_hand_type = hand_type(a_hand.clone());
        let b_hand_type = hand_type(b_hand.clone());
        if a_hand_type == b_hand_type {
            cmp_hand_values(
                a_hand
                    .iter()
                    .map(|c| card_char_to_value(*c))
                    .collect::<Vec<usize>>(),
                b_hand
                    .iter()
                    .map(|c| card_char_to_value(*c))
                    .collect::<Vec<usize>>(),
            )
        } else {
            let a_hand_type_value = hand_type_to_value(a_hand_type);
            let b_hand_type_value = hand_type_to_value(b_hand_type);
            a_hand_type_value.cmp(&b_hand_type_value)
        }
    });
    let mut total: usize = 0;
    for (i, entry) in entries.iter().enumerate() {
        let winnings = entry.bid * (i + 1);
        total += winnings;
    }
    total
}

#[derive(sscanf::FromSscanf, Debug)]
#[sscanf(format = "{hand} {bid}")]
struct Entry {
    hand: String,
    bid: usize,
}

fn card_char_to_value(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '*' => 0,
        i => i.to_digit(10).unwrap() as usize,
    }
}

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

fn hand_type_to_value(hand_type: HandType) -> usize {
    match hand_type {
        HandType::FiveOfAKind => 6,
        HandType::FourOfAKind => 5,
        HandType::FullHouse => 4,
        HandType::ThreeOfAKind => 3,
        HandType::TwoPair => 2,
        HandType::Pair => 1,
        HandType::HighCard => 0,
    }
}

fn hand_type(hand: Vec<char>) -> HandType {
    if hand.len() == 5 {
        let mut counts = vec![0; 15];
        for card in hand {
            counts[card_char_to_value(card)] += 1;
        }
        let jokers = counts[0];
        counts.remove(0);
        let mut counts = counts.iter().filter(|c| **c > 0).collect::<Vec<&usize>>();
        if counts.is_empty() {
            if jokers == 5 {
                return HandType::FiveOfAKind;
            } else if jokers == 4 {
                return HandType::FourOfAKind;
            } else if jokers == 3 {
                return HandType::ThreeOfAKind;
            } else if jokers == 2 {
                return HandType::Pair;
            } else {
                return HandType::HighCard;
            }
        }
        counts.sort();
        counts.reverse();
        let counts = counts.iter().map(|c| **c).collect::<Vec<usize>>();
        if counts[0] + jokers == 5 {
            HandType::FiveOfAKind
        } else if counts[0] + jokers == 4 {
            HandType::FourOfAKind
        } else if (counts[0] == 3 && counts[1] == 2)
            || (counts[0] == 2 && counts[1] == 2 && jokers == 1)
        {
            HandType::FullHouse
        } else if counts[0] + jokers == 3 {
            HandType::ThreeOfAKind
        } else if counts[0] == 2 && counts[1] == 2 {
            HandType::TwoPair
        } else if counts[0] + jokers == 2 {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    } else {
        panic!("Invalid hand size: {}", hand.len());
    }
}

fn cmp_hand_values(a: Vec<usize>, b: Vec<usize>) -> std::cmp::Ordering {
    for i in 0..a.len() {
        match a[i].cmp(&b[i]) {
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => continue,
        }
    }
    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn p2() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(part2(input), 5905);
    }

    #[test]
    fn p2_hand_types() {
        assert_eq!(hand_type("*22TT".chars().collect::<Vec<char>>()), HandType::FullHouse);
    }
}

