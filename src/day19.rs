use std::fs;

#[derive(Clone)]
enum Attribute {
    X,
    M,
    A,
    S,
    None,
}

#[derive(Clone)]
struct Filter {
    attribute: Attribute,
    filter: FilterFunc,
}

#[derive(Clone)]
enum FilterFunc {
    Always,
    Gt(usize),
    Lt(usize),
}

impl Filter {
    fn apply(&self, part: &Part) -> bool {
        let val = match self.attribute {
            Attribute::X => part.x,
            Attribute::M => part.m,
            Attribute::A => part.a,
            Attribute::S => part.s,
            Attribute::None => return true,
        };

        match self.filter {
            FilterFunc::Always => true,
            FilterFunc::Gt(n) => val > n,
            FilterFunc::Lt(n) => val < n,
        }
    }

    fn split_parts(&self, part: &PartPair) -> (PartPair, PartPair) {
        let mut accepted_part = part.clone();
        let mut rejected_part = part.clone();
        fn getter<'a>(part: &'a mut PartPair, attribute: &Attribute) -> &'a mut (usize, usize) {
            match attribute {
                Attribute::X => &mut part.x,
                Attribute::M => &mut part.m,
                Attribute::A => &mut part.a,
                Attribute::S => &mut part.s,
                Attribute::None => unreachable!(),
            }
        }

        match self.filter {
            FilterFunc::Always => {}
            FilterFunc::Gt(n) => {
                getter(&mut accepted_part, &self.attribute).0 = n + 1;
                getter(&mut rejected_part, &self.attribute).1 = n
            }
            FilterFunc::Lt(n) => {
                getter(&mut accepted_part, &self.attribute).1 = n - 1;
                getter(&mut rejected_part, &self.attribute).0 = n
            }
        };

        (accepted_part, rejected_part)
    }
}

#[derive(Clone)]
enum Destination {
    Accept,
    Reject,
    WorkFlow(String),
}

#[derive(Clone)]
struct Condition {
    filter: Filter,
    destination: Destination,
}

#[derive(Clone)]
struct WorkFlow {
    name: String,
    conditions: Vec<Condition>,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone)]
struct PartPair {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl std::fmt::Debug for PartPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(x:{}..{}, m:{}..{}, a:{}..{}, s:{}..{})",
            self.x.0, self.x.1, self.m.0, self.m.1, self.a.0, self.a.1, self.s.0, self.s.1,
        ))
    }
}

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input19.txt").expect("should've been able to read");
    let (workflows, parts) = parse_input(&input);

    let mut sum = 0;

    for part in parts {
        let mut curr_workflow = workflows
            .iter()
            .find(|workflow| &workflow.name == "in")
            .unwrap()
            .clone();

        loop {
            let mut processed = false;
            let mut new_workflow = None;
            for condition in &curr_workflow.conditions {
                if condition.filter.apply(&part) {
                    match &condition.destination {
                        Destination::Accept => {
                            processed = true;
                            sum += part.x + part.m + part.a + part.s;
                        }
                        Destination::Reject => processed = true,
                        Destination::WorkFlow(dest) => {
                            new_workflow = workflows.iter().find(|workflow| workflow.name == **dest)
                        }
                    };
                    break;
                }
            }
            if processed {
                break;
            }
            curr_workflow = new_workflow.unwrap().clone();
        }
    }

    println!("{}", sum);
}

fn parse_attribute(s: &str) -> Attribute {
    match s {
        "x" => Attribute::X,
        "m" => Attribute::M,
        "a" => Attribute::A,
        "s" => Attribute::S,
        _ => Attribute::None,
    }
}

fn parse_filter(s: &str) -> Filter {
    if let Some((comparison, value)) = s.split_once('<') {
        Filter {
            attribute: parse_attribute(comparison),
            filter: FilterFunc::Lt(value.parse().unwrap()),
        }
    } else if let Some((comparison, value)) = s.split_once('>') {
        Filter {
            attribute: parse_attribute(comparison),
            filter: FilterFunc::Gt(value.parse().unwrap()),
        }
    } else {
        Filter {
            attribute: Attribute::None,
            filter: FilterFunc::Always,
        }
    }
}

fn parse_destination(input: &str) -> Destination {
    match input {
        "A" => Destination::Accept,
        "R" => Destination::Reject,
        _ => Destination::WorkFlow(input.to_string()),
    }
}

fn parse_workflow(input: &str) -> WorkFlow {
    let (name, mut rest) = input.split_once('{').unwrap();
    rest = &rest[0..rest.len() - 1];
    let conditions = rest.split(',');
    let conditions = conditions
        .map(|cond| {
            if cond.find(':').is_none() {
                Condition {
                    filter: Filter {
                        filter: FilterFunc::Always,
                        attribute: Attribute::None,
                    },
                    destination: parse_destination(cond),
                }
            } else {
                let (filter, dest) = cond.split_once(':').unwrap();
                Condition {
                    filter: parse_filter(filter),
                    destination: parse_destination(dest),
                }
            }
        })
        .collect::<Vec<_>>();

    WorkFlow {
        name: name.to_string(),
        conditions,
    }
}

fn parse_part(input: &str) -> Part {
    let input = &input[1..input.len() - 1];
    let mut input = input
        .split(',')
        .map(|a| a.split_once('=').unwrap().1.parse().unwrap());

    Part {
        x: input.next().unwrap(),
        m: input.next().unwrap(),
        a: input.next().unwrap(),
        s: input.next().unwrap(),
    }
}

fn parse_input(input: &str) -> (Vec<WorkFlow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(parse_workflow);
    let parts = parts.lines().map(parse_part);

    (workflows.collect(), parts.collect())
}

fn parse_input_2(s: &str) -> Vec<WorkFlow> {
    let (workflows, _) = s.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(parse_workflow);

    workflows.collect()
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input19.txt").expect("should've been able to read");
    let workflows = parse_input_2(&input);
    let total_part = PartPair {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let mut workflow_part_pairs: Vec<(WorkFlow, PartPair)> = vec![(
        workflows
            .iter()
            .find(|workflow| &workflow.name == "in")
            .unwrap()
            .clone(),
        total_part,
    )];

    let mut accepted_ranges = Vec::new();
    while let Some((workflow, mut part)) = workflow_part_pairs.pop() {
        for condition in &workflow.conditions {
            let (accepted_part, rejected_part) = condition.filter.split_parts(&part);

            match &condition.destination {
                Destination::Accept => {
                    accepted_ranges.push(accepted_part);
                }
                Destination::Reject => {}
                Destination::WorkFlow(dest) => {
                    workflow_part_pairs.push((
                        workflows
                            .iter()
                            .find(|workflow| workflow.name == *dest)
                            .unwrap()
                            .clone(),
                        accepted_part,
                    ));
                }
            }
            part = rejected_part;
        }
    }

    fn volume(part: &PartPair) -> usize {
        fn range_width(range: (usize, usize)) -> usize {
            range.1 - range.0 + 1
        }

        range_width(part.x) * range_width(part.m) * range_width(part.a) * range_width(part.s)
    }

    println!("{}", accepted_ranges.iter().map(volume).sum::<usize>());
}
