use std::fs::read_to_string;
use std::collections::{HashMap, VecDeque};
use aoc::utils::lcm;

fn main() {
    let mut modules = parse_file("./input.data");
    // dbg!(&modules);
    
    // PART 1
    let mut pulses = [0; 2]; // [num high, num low]
    let mut queue = VecDeque::<(String, String, bool)>::new();
    for _ in 0..1000 {
        queue.push_back(("".to_owned(), "broadcaster".to_owned(), false));

        while let Some((sender, reciever, pulse)) = queue.pop_front() {
            if pulse == true {pulses[0] += 1}
            else {pulses[1] += 1}
            
            match modules.get_mut(&reciever).unwrap().pulse_recieve(&sender, pulse) {
                Some(next_pulse) => {
                    // send pulses to queue
                    for next_reciever in modules[&reciever].output_modules.clone() {
                        queue.push_back((reciever.clone(), next_reciever, next_pulse));
                    }
                }
                None => {},
            }
        }
    }
    
    // dbg!(&pulses);
    println!("Result part 1: {}", pulses[0] * pulses[1]);


    // PART 2
    // Reset all modules to their default
    let name_list: Vec<String> = modules.keys().map(|s| s.to_owned()).collect();
    for name in &name_list {
        let module = modules.get_mut(name).unwrap();
        module.state = module.state.iter().map(|_| false).collect();
    }

    // There is only a single module inputting to "rx"
    // And that module is a conjunction
    if modules["rx"].input_modules.len() != 1 {panic!("Not exactly one input")}
    let primary_conjunction = modules["rx"].input_modules[0].clone();
    if modules[&primary_conjunction].module_type != "conjunction" {panic!("Not a conjunction")}

    // Will search for the period of switching of the inputs to the primary conjunction
    let mut listeners = modules[&primary_conjunction].input_modules.clone();
    let mut periods = Vec::new();
    let mut button_presses: u64 = 0;
    let mut queue = VecDeque::<(String, String, bool)>::new();
    'outer: loop {
        queue.push_back(("".to_owned(), "broadcaster".to_owned(), false));
        button_presses += 1;

        while let Some((sender, reciever, pulse)) = queue.pop_front() {
            // check if modules listened to has changed state
            match (listeners.iter().position(|m| m==&sender), pulse) {
                (Some(index), true) => {
                    periods.push(button_presses);
                    listeners.remove(index);
                    if listeners.len() == 0 {break 'outer}
                },
                _ => {},
            }

            match modules.get_mut(&reciever).unwrap().pulse_recieve(&sender, pulse) {
                Some(next_pulse) => {
                    // send pulses to queue
                    for next_reciever in modules[&reciever].output_modules.clone() {
                        queue.push_back((reciever.clone(), next_reciever, next_pulse));
                    }
                }
                None => {},
            }
        }
    }

    // dbg!(&periods);
    println!("Result part 2: {}", periods.iter().fold(1, |res, p| lcm(res, *p)));
}

#[derive(Debug, Default, Clone)]
struct Module {
    module_type: String,
    state: Vec<bool>,
    input_modules: Vec<String>,
    output_modules: Vec<String>,
}

impl Module {
    fn from_prefix(prefix: char) -> Self {
        let mut result = Self::default();
        match prefix { // luckily bools default to false
            '%' => {
                result.module_type = "flip-flop".to_string();
                result.state.push(false);
            },
            '&' => result.module_type = "conjunction".to_string(),
            'B' => result.module_type = "broadcaster".to_string(),
            _ => result.module_type = "untyped".to_string(),
        };
        return result
    }

    fn add_input(&mut self, module_name: String) {
        if module_name == "rx" {dbg!(&self.module_type);}
        self.input_modules.push(module_name);
        if self.module_type == "conjunction" {
            self.state.push(false);
        }
    }

    fn pulse_recieve(&mut self, sender: &str, pulse: bool) -> Option<bool> {        
        match self.module_type.as_str() {
            "conjunction" => {
                let index = self.input_modules.iter().position(|m| m == sender).unwrap();
                self.state[index] = pulse;
                return Some(!self.state.iter().all(|b| *b));
            },
            "flip-flop" => {
                if pulse == false {
                    self.state[0] = !self.state[0];
                    return Some(self.state[0]);
                } else {
                    return None;
                }
            },
            "broadcaster" => {
                if pulse == false {
                    return Some(false);
                } else {
                    return None;
                }
            }
            "untyped" | _ => return None,
        }
    }
}

fn parse_file(filepath: &str) -> HashMap<String, Module> {
    let input = read_to_string(filepath).unwrap();
    let mut lines = input.trim().split('\n').map(|x| x.replace(" ", ""));
    
    // First we build all modules with no inputs
    let mut modules: HashMap<String, Module> = HashMap::new();

    // I have manually put "broadcaster" to first line
    let broadcaster_outputs = lines.next().unwrap()
        .replace("broadcaster->", "").split(',')
        .map(|s| s.to_string())
        .collect();
    let mut broadcaster = Module::from_prefix('B');
    broadcaster.output_modules = broadcaster_outputs;
    modules.insert("broadcaster".to_owned(), broadcaster);
    
    // All other modules
    for line in lines {
        let (module_type, outputs) = line.split_once("->").unwrap();
        let mut chars = module_type.chars();
        let prefix = chars.next().unwrap();
        let name: String = chars.collect();
        let output_modules = outputs.split(',').map(|s| s.to_string()).collect();

        let mut module = Module::from_prefix(prefix);
        module.output_modules = output_modules;
        modules.insert(name, module);

    }

    // Now we add input connections
    let name_list: Vec<String> = modules.keys().map(|s| s.to_owned()).collect();
    for sender in name_list {
        let receivers = &modules[&sender].output_modules.clone();
        for receiver in receivers {
            match modules.get_mut(receiver) {
                Some(module) => {
                    module.add_input(sender.clone());
                },
                None => {
                    let mut new_module = Module::from_prefix('?');
                    new_module.add_input(sender.clone());
                    modules.insert(receiver.clone(), new_module);
                },
            }
        }
    }

    return modules
}