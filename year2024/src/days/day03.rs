use regex::Regex;

pub fn run(use_test_input: bool) {
    let input = super::read_input(3, use_test_input);
    let re = Regex::new(r"(mul\(\d+,\d+\))|(don't)|(do)").unwrap();
    let instructions: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();

    // Run instructions
    let mut part1 = 0;
    let mut part2 = 0;
    let mut enabled = true;
    for instr in instructions {
        match instr {
            "do" => enabled = true,
            "don't" => enabled = false,
            _ => {
                let val = instr
                    .replace("mul(", "")
                    .replace(")", "")
                    .split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .fold(1, |acc, num| acc * num);
                part1 += val;
                part2 += val * enabled as u32
            }
        }
    }

    println!("Result part 1: {part1}");
    println!("Result part 2: {part2}");
}
