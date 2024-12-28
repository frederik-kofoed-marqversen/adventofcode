use regex::Regex;
use std::collections::{HashMap, VecDeque};

pub fn run(use_test_input: bool) {
    let input = super::read_input(24, use_test_input);
    let args = input.split_once("\n\n").unwrap();

    // vec![in1, gate, in2, "->", out]
    let mut gates: Vec<Vec<&str>> = args
        .1
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();

    // PART 1
    let mut x = Vec::new();
    let mut y = Vec::new();
    for line in args.0.lines() {
        let (wire, val) = line.split_once(": ").unwrap();
        if &wire[0..1] == "x" {
            x.push(val == "1");
        } else {
            y.push(val == "1");
        }
    }

    println!(
        "Result part 1: {}",
        run_device(bitvec_to_u64(&x), bitvec_to_u64(&y), &gates)
    );

    // PART 2
    // Fix found swaps
    let swaps = vec![
        "shj", "z07",
        "wkb", "tpk",
        "pfn", "z23",
        "kcd", "z27"
    ];
    for gate in gates.iter_mut() {
        if let Some(index) = swaps.iter().position(|&s| s == gate[4]) {
            if index % 2 == 0 {
                gate[4] = swaps[index + 1];
            } else {
                gate[4] = swaps[index - 1]
            }
        }
    }

    let mut aliases: HashMap<&str, String> = HashMap::new();
    // Built alias table
    let mut redo = true;
    while redo {
        redo = false;
        for gate in &gates {
            // Get wire aliases
            let wires: Vec<String> = [0, 2, 4]
                .iter()
                .map(|&i| aliases.get(gate[i]).cloned().unwrap_or(gate[i].to_string()))
                .collect();

            // Check if output wire can be given an alias
            if !aliases.contains_key(&gate[4]) {
                if let Some(alias) =
                    out_wire_alias(vec![&wires[0], gate[1], &wires[1], "->", &wires[2]])
                {
                    aliases.insert(gate[4], alias);
                    redo = true;
                }
            }
        }
    }

    // Sort gates by output wire name for easy reading
    let mut gates_aliased: Vec<Vec<&str>> = gates
        .iter()
        .map(|gate| {
            let mut aliased: Vec<&str> = gate.iter()
                .map(|&s| aliases.get(s).map_or(s, |ss| ss.as_str()))
                .collect();
            (aliased[0], aliased[2]) = (aliased[0].min(aliased[2]), aliased[0].max(aliased[2]));
            aliased
        })
        .collect();
    gates_aliased.sort_by_key(|v| v[4]);
    for gate in gates_aliased {
        println!("{}", gate.join(" "));
    }

    // Print the names of the wires with certain aliases
    for alias in ["DIRECT_ADD16", "DIRECT_CARRY16"] {
        dbg!(aliases.iter().find(|(_, other)| other == &alias));
    }

    // Compute final result when swaps have been found
    let mut swaps = swaps;
    swaps.sort();
    println!(r"Not the prettiest solution... ¯\_(ツ)_/¯");
    println!("Result part 2: {}", swaps.join(","));
}

fn out_wire_alias(mut gate: Vec<&str>) -> Option<String> {
    // Sort input wires alphabetically
    (gate[0], gate[2]) = (gate[0].min(gate[2]), gate[0].max(gate[2]));

    let re = Regex::new(r"^(\D+)(\d*)$").unwrap();
    let (_, [name1, num1]) = re.captures(&gate[0]).unwrap().extract();
    let (_, [name2, num2]) = re.captures(&gate[2]).unwrap().extract();
    let gate_type = gate[1];

    if name1 == "x" && name2 == "y" && num1 == num2 {
        if num1 == "00" {
            if gate_type == "AND" {
                return Some(format!("FINAL_CARRY{num1}"));
            } else {
                return None;
            }
        }
        if gate_type == "AND" {
            return Some(format!("DIRECT_CARRY{num1}"));
        }
        if gate_type == "XOR" {
            return Some(format!("DIRECT_ADD{num1}"));
        }
    }

    if name1 == "DIRECT_ADD" && name2 == "FINAL_CARRY" && num1 > num2 {
        if gate_type == "AND" {
            return Some(format!("MID_CARRY{num1}"));
        }
        if gate_type == "XOR" {
            // This should be the final output bit which we do not rename
            return None
        }
    }

    if name1 == "DIRECT_CARRY" && name2 == "MID_CARRY" && num1 == num2 && gate_type == "OR" {
        return Some(format!("FINAL_CARRY{num1}"));
    }

    return None;
}

fn u64_to_bitvec(num: u64, length: usize) -> Vec<bool> {
    (0..length).map(|i| (num >> i) & 1 == 1).collect()
}

fn bitvec_to_u64(vec: &Vec<bool>) -> u64 {
    vec.iter()
        .enumerate()
        .map(|(i, b)| if *b { 1 << i } else { 0 })
        .sum()
}

fn run_device(x: u64, y: u64, gates: &Vec<Vec<&str>>) -> u64 {
    let mut values = HashMap::new();
    for (i, val) in u64_to_bitvec(x, 45).into_iter().enumerate() {
        values.insert(format!("x{i:02}"), val);
    }
    for (i, val) in u64_to_bitvec(y, 45).into_iter().enumerate() {
        values.insert(format!("y{i:02}"), val);
    }

    let mut queue = VecDeque::from_iter(gates);
    while let Some(gate) = queue.pop_front() {
        let ins = (values.get(gate[0]), values.get(gate[2]));
        if ins.0.is_some() && ins.1.is_some() {
            let ins = (ins.0.unwrap(), ins.1.unwrap());
            let out = match gate[1] {
                "AND" => ins.0 & ins.1,
                "OR" => ins.0 | ins.1,
                "XOR" => ins.0 ^ ins.1,
                g => panic!("Gate: {g} not recognised!"),
            };
            values.insert(gate[4].to_string(), out);
        } else {
            queue.push_back(gate);
        }
    }

    let output_bitvec = (0..46)
        .map(|i| *values.get(&format!("z{i:02}")).unwrap_or(&false))
        .collect();

    return bitvec_to_u64(&output_bitvec);
}
