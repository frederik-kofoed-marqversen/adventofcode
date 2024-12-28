use cached::proc_macro::cached;

type DataStruct = Vec<(Vec<char>, Vec<usize>)>;

pub fn run(use_test_input: bool) {
    let input = super::read_input(12, use_test_input);
    let input = parse_input(&input);
    // dbg!(&input);

    // PART 1
    let mut num_permutations = Vec::new();
    for (sequence, groups) in &input {
        num_permutations.push(combinations(sequence.clone(), groups.clone()));
    }
    // dbg!(&num_permutations);

    println!("Result part 1: {}", num_permutations.iter().sum::<u64>());

    // PART 2
    // The solution turns out to be caching the recursive function!
    let mut num_permutations = Vec::new();
    for (mut sequence, mut groups) in input {
        sequence.push('?');
        sequence = sequence.repeat(5);
        sequence.pop();

        groups = groups.repeat(5);
        num_permutations.push(combinations(sequence, groups));
    }
    // dbg!(&num_permutations);
    println!("Result part 2: {}", num_permutations.iter().sum::<u64>());
}

#[cached]
fn combinations(sequence: Vec<char>, groups: Vec<usize>) -> u64 {
    if groups.len() == 0 {
        return if sequence.iter().any(|x| x == &'#') {
            0
        } else {
            1
        };
    }

    let group_length = groups[0];
    let mut result = 0;
    for i in 0..sequence.len() {
        // let group be sequence[start..=end]
        let (start, end) = (i, i + group_length - 1);

        if (start > 0 && sequence[start - 1] == '#') || end >= sequence.len() {
            // group pushed too far right
            break;
        }
        if sequence[start..=end].iter().any(|x| x == &'.')
            || (end + 1 < sequence.len() && sequence[end + 1] == '#')
        {
            // group cannot be here -> step right
            continue;
        }

        // group can be here
        result += combinations(
            sequence[(end + 2).min(sequence.len())..].to_vec(),
            groups[1..].to_vec(),
        );
    }

    return result;
}

fn parse_input(input: &str) -> DataStruct {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut args = line.split_whitespace();
        let characters = args.next().unwrap().chars().collect();
        let groups = args
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        result.push((characters, groups));
    }

    return result;
}
