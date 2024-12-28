type Card = [Vec<u32>; 2];
type DataStruct = Vec<Card>;

pub fn run(use_test_input: bool) {
    let input = super::read_input(4, use_test_input);
    let cards = parse_input(&input);

    // PART 1
    let mut card_values = Vec::new();
    for card in cards.iter() {
        let num_wins = num_of_wins(&card);
        if num_wins > 0 {
            card_values.push(2_u32.pow(num_wins - 1));
        } else {
            card_values.push(0);
        }
    }
    println!("Result part 1: {}", card_values.iter().sum::<u32>());

    // PART 2
    let mut num_of_cards = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let num_wins = num_of_wins(&card) as usize;
        for j in i + 1..=i + num_wins {
            num_of_cards[j] += num_of_cards[i];
        }
        // dbg!(&num_wins, &num_of_cards);
    }
    println!("Result part 2: {}", num_of_cards.iter().sum::<u32>());
}

fn num_of_wins(card: &Card) -> u32 {
    let winning_numbers = &card[0];
    let your_numbers = &card[1];

    let mut result = 0;
    for num in your_numbers {
        if winning_numbers.contains(num) {
            result += 1;
        }
    }
    return result;
}

fn parse_input(input: &str) -> DataStruct {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut args = line.split([':', '|']).map(|x| x.trim());
        let _: u32 = args
            .next()
            .unwrap()
            .replace("Card ", "")
            .trim()
            .parse()
            .unwrap();
        let mut card = [Vec::new(), Vec::new()];
        for (i, arg) in args.enumerate() {
            let numbers: Vec<u32> = arg
                .split_whitespace()
                .map(|x| x.trim().parse().unwrap())
                .collect();
            card[i] = numbers;
        }
        result.push(card);
    }

    return result;
}
