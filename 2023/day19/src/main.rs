use std::fs::read_to_string;
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let input = read_to_string("./input.data")?;
    let mut args = input.trim().split("\n\n");

    let mut workflows = HashMap::new();
    for workflow in args.next().unwrap().split('\n') {
        let (name, rules) = parse_workflow(workflow);
        workflows.insert(name, rules);
    }
    
    
    // PART 1
    let parts = args.next().unwrap().split('\n').map(|x| parse_part(x).unwrap());
    
    let mut accepted_parts = Vec::new();
    for part in parts {
        let mut destination = "in";
        loop {
            let workflow = match destination {
                "R" => break,
                "A" => {
                    accepted_parts.push(part);
                    break;
                },
                _ => &workflows[destination],
            };
            for rule in workflow {
                match part.evaluate_rule(rule) {
                    Some(new_destination) => {
                        destination = new_destination;
                        break;
                    },
                    None => continue,
                }
            }
        }
    }
        
    let ratings = accepted_parts.iter().map(|part| part.x + part.m + part.a + part.s);
    println!("Result part 1: {}", ratings.sum::<u64>());




    // PART 2
    let input_space = Region4{data: [Interval{lower: 1, upper: 4_000}; 4]};
    
    let mut accepted_regions: Vec<Region4> = Vec::new();
    let mut queue = vec![("in", input_space)];
    // let mut step = 0;
    'outer: while let Some((destination, mut region)) = queue.pop() {        
        if region.volume() == 0 || destination == "R" {
            continue;
        }
        if destination == "A" {
            accepted_regions.push(region);
            continue;
        }
        
        for rule in &workflows[destination] {
            match rule.split_once(':') {
                Some((comparison, new_destination)) => {
                    let mut chars = comparison.chars();
                    let i = category_to_index(chars.next().unwrap()).unwrap();
                    let comparator = chars.next().unwrap();
                    let bound: u64 = chars.collect::<String>().parse().unwrap();
                    
                    let (positive, negative) = match comparator {
                        '>' => (
                            Interval::new(bound + 1, u64::MAX),
                            Interval::new(0, bound)
                        ),
                        '<' => (
                            Interval::new(0, bound - 1),
                            Interval::new(bound, u64::MAX)
                        ),
                        _ => {
                            println!("\n!!!Something went wrong!!!\n");
                            break 'outer;
                        }
                    };

                    // this region is accepted by the current rule and thus is sent to `new_destination`
                    let mut new_region = region.clone();
                    new_region.data[i] = new_region.data[i].intersection(&positive);
                    queue.push((new_destination, new_region));
                    // this remaining goes on to be evaluated by the next rule in the current workflow
                    region.data[i] = region.data[i].intersection(&negative);
                },
                None => {
                    // the region is simply passed on to the next workflow
                    queue.push((&rule, region));
                    break;
                }
            }
        }
    }
    
    // The regions will be disjoint since they are constructed by sequencially slicing the input space.
    let total_volume = accepted_regions.iter().map(|r| r.volume()).sum::<u64>();
    println!("Result part 2: {}", total_volume);

    Ok(())
}

fn category_to_index(c: char) -> Option<usize> {
    match c {
        'x' => Some(0),
        'm' => Some(1),
        'a' => Some(2),
        's' => Some(3),
        _ => None
    }
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    lower: u64,
    upper: u64,
}

impl Interval {
    fn new(lower: u64, upper: u64) -> Self {
        Self {lower, upper}
    }

    fn intersection(&self, other: &Self) -> Self {
        return Self {
            lower: self.lower.max(other.lower),
            upper: self.upper.min(other.upper),
        }
    }

    fn volume(&self) -> u64 {
        if self.lower > self.upper {
            0
        } else {
            self.upper - self.lower + 1
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Region4 {
    data: [Interval; 4],
}

impl Region4 {
    fn volume(&self) -> u64 {
        self.data.iter().fold(1, |prod, interval| prod * interval.volume())
    }
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    // a<2006:qkq || rfg
    fn evaluate_rule<'a>(&self, rule: &'a str) -> Option<&'a str> {
        match rule.split_once(':') {
            Some((comparison, destination)) => {
                if evaluate_comparison(comparison, &self).unwrap() {
                    return Some(destination)
                } else {
                    return None
                }
            },
            None => {
                return Some(rule)
            }
        }
    }
}

// a<2006
fn evaluate_comparison(comparison: &str, part: &Part) -> Option<bool> {
    let chars: Vec<char> = comparison.chars().collect();
    
    let right_num: u64 = chars[2..].iter().collect::<String>().parse().unwrap();
    let left_num = match chars[0] {
        'x' => part.x,
        'm' => part.m,
        'a' => part.a,
        's' => part.s,
        _ => return None,
    };

    return match chars[1] {
        '>' => Some(left_num > right_num),
        '<' => Some(left_num < right_num),
        _ => None,
    }
}

// {x=787,m=2655,a=1222,s=2876}
fn parse_part(part: &str) -> Option<Part> {
    let mut chars: Vec<char> = part.trim().chars().collect();
    
    // remove baces
    chars.remove(0);
    chars.pop();
    
    let mut part = Part{x:0, m:0, a:0, s:0};
    for category in chars.split(|&x| x==',') {
        let num: u64 = match category[2..].iter().collect::<String>().parse(){
            Ok(n) => n,
            _ => return None,
        };
        match category[0] {
            'x' => part.x = num,
            'm' => part.m = num,
            'a' => part.a = num,
            's' => part.s = num,
            _ => return None,
        }
    }

    return Some(part)
}

// px{a<2006:qkq,m>2090:A,rfg}
fn parse_workflow(workflow: &str) -> (&str, Vec<String>) {
    let (name, rest) = workflow.split_once(|x| x=='{').unwrap();
    
    let mut inner: Vec<char> = rest.chars().collect();
    inner.pop(); // remove trainling '}'
    let rules = inner.split(|&x| x==',').map(|x| x.iter().collect::<String>()).collect();    
    
    return (name, rules)
}