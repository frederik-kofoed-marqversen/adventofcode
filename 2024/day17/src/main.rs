fn main() {
    // INPUT
    let program = vec![2, 4, 1, 2, 7, 5, 1, 3, 4, 4, 5, 5, 0, 3, 3, 0];

    // PART 1
    let output = run_program(&program, 48744869);
    println!(
        "Result part 1: {}",
        output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    // PART 2
    /*By inspecting the program one notice that the initial value of the B and C
    registers do not matter, as they are overwritten before accessed. Looking at
    the output for increasing input number one notice that there is a certain base
    8 logic going on. The second number change exactly every 8 steps, the third
    every 8*8 steps etc.. This leads to the following solution: */

    let mut number = vec![0; program.len()];
    number[program.len() - 1] = 1;
    let mut output = run_program(&program, eval_base_8(&number));
    let mut pointer = program.len() - 1;

    while output != program {
        if output[pointer] == program[pointer] {
            pointer -= 1;
        } else if number[pointer] == 7 {
            number[pointer] = 0;
            pointer += 1;
            number[pointer] += 1;
        } else {
            number[pointer] += 1;
        }

        output = run_program(&program, eval_base_8(&number));
    }

    println!("Result part 2: {}", eval_base_8(&number));
}

fn eval_base_8(digits: &[u64]) -> u64 {
    // computes base 8 number: &[1, 2, 3] => 1 + 2*8^1 + 3*8^2
    digits
        .iter()
        .enumerate()
        .fold(0, |acc, (i, n)| acc + n * 8_u64.pow(i as u32))
}

fn eval_combo_operand(operand: u64, registers: &(u64, u64, u64)) -> u32 {
    return match operand {
        0 | 1 | 2 | 3 => operand,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        _ => panic!("Not a valid operand"),
    } as u32;
}

fn run_program(program: &Vec<u64>, num: u64) -> Vec<u64> {
    let (mut a, mut b, mut c) = (num, 0, 0);

    let mut output = Vec::new();
    let mut pointer = 0;

    loop {
        if pointer >= program.len() {
            break;
        }
        let (opcode, operand) = (program[pointer], program[pointer + 1]);
        match opcode {
            0 => a = a / 2_u64.pow(eval_combo_operand(operand, &(a, b, c))),
            1 => b = b ^ operand,
            2 => b = (eval_combo_operand(operand, &(a, b, c)) % 8) as u64,
            3 => {
                if a != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => b = b ^ c,
            5 => output.push((eval_combo_operand(operand, &(a, b, c)) % 8) as u64),
            6 => b = a / 2_u64.pow(eval_combo_operand(operand, &(a, b, c))),
            7 => c = a / 2_u64.pow(eval_combo_operand(operand, &(a, b, c))),
            _ => panic!("Unknown opcode: {}", opcode),
        }
        pointer += 2;
    }

    return output;
}
