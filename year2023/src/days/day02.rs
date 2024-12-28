pub fn run(use_test_input: bool) {
    let input = super::read_input(2, use_test_input);

    let mut valid_games_sum: u32 = 0;
    let mut power_sum: u32 = 0;
    for line in input.lines() {
        let mut args = line.split(":");
        let id: u32 = args
            .next()
            .unwrap()
            .replace("Game", "")
            .trim()
            .parse()
            .unwrap();
        let game = args.next().unwrap();
        let pulls = parse_game(game).unwrap();

        let mut valid_game = true;
        let (mut red_min, mut green_min, mut blue_min) = (0, 0, 0);
        for (red, green, blue) in pulls {
            if red > 12 || green > 13 || blue > 14 {
                valid_game = false;
            }
            red_min = u32::max(red_min, red);
            green_min = u32::max(green_min, green);
            blue_min = u32::max(blue_min, blue);
        }
        if valid_game {
            valid_games_sum += id;
        }
        let power = red_min * green_min * blue_min;
        // dbg!(power);
        power_sum += power;
    }
    println!("Result part 1: {valid_games_sum}");
    println!("Result part 2: {power_sum}");
}

fn parse_game(game: &str) -> Result<Vec<(u32, u32, u32)>, ()> {
    let mut pulls = game.split(";");
    let mut result = Vec::new();
    while let Some(pull) = pulls.next() {
        let mut balls = (0, 0, 0);

        for string in pull.split(",") {
            let mut args = string.trim().split(" ");
            let num: u32 = args.next().unwrap().parse().unwrap();
            let colour = args.next().unwrap().trim();
            match colour {
                "red" => balls.0 = num,
                "green" => balls.1 = num,
                "blue" => balls.2 = num,
                _ => return Err(()),
            }
        }
        result.push(balls);
    }
    return Ok(result);
}
