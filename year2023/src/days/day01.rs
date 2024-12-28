pub fn run(use_test_input: bool) {
    let input = super::read_input(1, use_test_input);

    let mut sum: u32 = 0;
    for line in input.lines() {
        let ints: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
        let int = format!("{}{}", ints[0], ints.last().unwrap());
        let num: u32 = int.parse::<u32>().unwrap();
        sum += num;
    }
    println!("Result part 1: {sum}");

    // PART TWO
    let patterns = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];

    let mut sum: u32 = 0;
    for line in input.lines() {
        let string = line;

        let mut index1 = 10000000;
        let mut index2 = 0;
        let mut num1 = "";
        let mut num2 = "";
        for pattern in patterns {
            match string.find(pattern) {
                None => (),
                Some(i) => {
                    if i <= index1 {
                        index1 = i;
                        num1 = pattern;
                    }
                }
            };

            match string.rfind(pattern) {
                None => (),
                Some(i) => {
                    if i >= index2 {
                        index2 = i;
                        num2 = pattern;
                    }
                }
            };
        }

        let int = to_num(&format!("{num1}{num2}"));
        let num = int.parse::<u32>().unwrap();
        sum += num;
    }
    println!("Result part 2: {}", sum);
}

fn to_num(string: &str) -> String {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut result = string.to_string();
    for (i, word) in words.into_iter().enumerate() {
        result = result.replace(word, i.to_string().as_str())
    }
    return result;
}
