use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

enum Node<'a> {
    FlipFlop(bool),
    AndGate(HashMap<&'a str, bool>),
    Broadcaster,
}

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input20.txt").expect("should've been able to read");
    let mut connections = HashMap::new();
    let mut state = HashMap::new();

    for line in input.trim_end().split('\n') {
        let (src, rest) = line.split_once(" -> ").unwrap();
        let target_nodes = rest.split(", ").collect::<Vec<_>>();
        let (node, state_type) = match src.as_bytes()[0] as char {
            '%' => (&src[1..], Node::FlipFlop(false)),
            '&' => (&src[1..], Node::AndGate(HashMap::new())),
            'b' => (src, Node::Broadcaster),
            _ => unreachable!(),
        };
        connections.insert(node, target_nodes);
        state.insert(node, state_type);
    }

    let mut recv_node = "";
    for (&node, target_nodes) in &connections {
        for &target_node in target_nodes {
            match state.get_mut(target_node) {
                Some(Node::AndGate(and_map_gate)) => {
                    and_map_gate.insert(node, false);
                }
                Some(_) => {}
                None => recv_node = node,
            }
        }
    }
    let mut cycles = match &state[recv_node] {
        Node::AndGate(and_map_gate) => and_map_gate
            .iter()
            .map(|(&node, _)| (node, None))
            .collect::<HashMap<_, _>>(),
        _ => unreachable!(),
    };

    let mut p1 = [0, 0];
    let mut queue = VecDeque::new();
    'outer: for time_step in 1.. {
        queue.push_back(("broadcaster", "button", false));
        while let Some((node, prev, high)) = queue.pop_front() {
            if time_step <= 1000 {
                p1[high as usize] += 1;
            }
            if high && node == recv_node {
                let val = cycles.get_mut(prev).unwrap();
                if val.is_none() {
                    *val = Some(time_step);
                    if cycles.values().all(|opt| opt.is_some()) {
                        break 'outer;
                    }
                }
            }
            let pulse = match state.get_mut(node) {
                Some(Node::FlipFlop(_)) if high => continue,
                Some(Node::FlipFlop(on)) => {
                    *on = !*on;
                    *on
                }
                Some(Node::AndGate(and_map_gate)) => {
                    and_map_gate.insert(prev, high);
                    and_map_gate.values().any(|&bit| !bit)
                }
                Some(Node::Broadcaster) => false,
                None => continue,
            };
            queue.extend(
                connections[node]
                    .iter()
                    .map(|&target_node| (target_node, node, pulse)),
            );
        }
    }
    println!(
        "{} {}",
        p1[0] * p1[1],
        cycles.values().map(|opt| opt.unwrap()).product::<usize>()
    )
}

pub(crate) fn problem2() {
    println!("not implemented");
}
