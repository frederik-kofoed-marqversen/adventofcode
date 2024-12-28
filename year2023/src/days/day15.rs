type Lens = (String, usize);

pub fn run(use_test_input: bool) {
    let input = super::read_input(15, use_test_input);
    let steps: Vec<String> = input.trim().split(',').map(|x| x.to_string()).collect();

    dbg!(hash(&"HASH"));

    // PART 1
    let numbers: Vec<usize> = steps.iter().map(|step| hash(step)).collect();
    // dbg!(&numbers);
    println!("Result part 1: {}", numbers.iter().sum::<usize>());

    // PART 2
    let mut hashmap = vec![Vec::<Lens>::new(); 256];
    for step in steps {
        let mut args = step.split_inclusive(|c| ['-', '='].contains(&c));
        let mut chars: Vec<char> = args.next().unwrap().chars().collect();
        let operation = chars.pop().unwrap();
        let label = String::from_iter(chars);
        let box_num = hash(&label);
        match operation {
            '=' => {
                let lens = (label, args.next().unwrap().parse::<usize>().unwrap());
                match hashmap[box_num].iter().position(|x| lens.0 == x.0) {
                    Some(index) => {
                        hashmap[box_num][index] = lens;
                    }
                    None => {
                        hashmap[box_num].push(lens);
                    }
                }
            }
            '-' | _ => match hashmap[box_num].iter().position(|lens| lens.0 == label) {
                Some(index) => {
                    hashmap[box_num].remove(index);
                }
                None => {}
            },
        }
        // dbg!(&step);
        // for b in &hashmap {
        //     if !b.is_empty() {
        //         dbg!(&b);
        //     }
        // }
    }

    let focusing_powers: Vec<usize> = hashmap
        .iter()
        .enumerate()
        .map(|(box_index, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(move |(lens_index, lens)| lens.1 * (lens_index + 1) * (box_index + 1))
        })
        .flatten()
        .collect();
    // dbg!(&focusing_powers);

    println!("Result part 2: {}", focusing_powers.iter().sum::<usize>());
}

fn hash(string: &str) -> usize {
    string
        .chars()
        .fold(0, |res, c| ((res + c as usize) * 17) % 256)
}
