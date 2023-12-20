use std::collections::HashMap;

mod consts;

pub fn accepted_parts() -> usize {
    let mut iter = consts::INPUT.trim().split("\n\n");
    let workflows_vec: Vec<Workflow> = iter.next().unwrap().lines().map(Workflow::new).collect();
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflows_vec {
        workflows.insert(workflow.name.clone(), workflow);
    }
    let parts: Vec<Part> = iter.next().unwrap().lines().map(Part::new).collect();

    let mut result: usize = 0;
    for part in parts {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            let dest = workflow.run_part(&part);
            if dest == "A" {
                result += part.count();
                break;
            }
            if dest == "R" {
                break;
            }
            workflow = workflows.get(&dest).unwrap();
        }
    }
    result
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(line: &str) -> Self {
        let mut iter = line.split("{");
        let name = iter.next().unwrap().to_string();
        let mut rules: Vec<Rule> = Vec::new();
        for part_str in iter.next().unwrap().trim_end_matches('}').split(',') {
            rules.push(Rule::new(part_str));
        }
        Self {
            name,
            rules
        }
    }

    fn run_part(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.attr.is_none() {
                return rule.dest.clone();
            }
            let attr = rule.attr.unwrap();
            let comparator = rule.comparator.unwrap();
            let val = rule.val.unwrap();
            let eval = match attr {
                'x' => cmp_part(part.x, &comparator, val),
                'm' => cmp_part(part.m, &comparator, val),
                'a' => cmp_part(part.a, &comparator, val),
                's' => cmp_part(part.s, &comparator, val),
                _ => panic!("run_part: Unknown attr: {attr}")
            };
            if eval { return rule.dest.clone(); }
        }
        panic!("run_part: No rule found! Part: {part:?}, Workflow: {self:?}");
    }
}

#[derive(Debug)]
struct Rule {
    attr: Option<char>,
    comparator: Option<char>,
    val: Option<usize>,
    dest: String,
}

impl Rule {
    fn new(input: &str) -> Self {
        if !input.contains(':') {
            return Self {
                attr: None,
                comparator: None,
                val: None,
                dest: input.to_string()
            }
        }
        let attr = input.chars().nth(0).unwrap();
        let comparator = input.chars().nth(1).unwrap();
        let mut iter = input[2..].split(':');
        let val = iter.next().unwrap().parse::<usize>().unwrap();
        let dest = iter.next().unwrap().to_string();
        Self {
            attr: Some(attr),
            comparator: Some(comparator),
            val: Some(val),
            dest
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(input: &str) -> Self {
        let mut result = Self {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        let input = input.trim_start_matches('{').trim_end_matches('}');
        let mut iter = input.split(',');
        for part in iter {
            let mut iter = part.split('=');
            let attr = iter.next().unwrap();
            let val = iter.next().unwrap().parse::<usize>().unwrap();
            match attr {
                "x" => result.x = val,
                "m" => result.m = val,
                "a" => result.a = val,
                "s" => result.s = val,
                _ => panic!("Invalid attribute: {}", attr)
            }
        }
        result
    }

    fn count(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn cmp_part(val1: usize, comparator: &char, val2: usize) -> bool {
    match comparator {
        '>' => val1 > val2,
        '<' => val1 < val2,
        _ => panic!("Invalid comparator: {}", comparator)
    }
}