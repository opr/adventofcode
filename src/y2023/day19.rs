use crate::input_reader::input_reader::read_input_for_day;
use log::debug;
use std::collections::{HashMap, HashSet};
use std::panic::panic_any;

#[derive(Clone, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
#[derive(Clone, Debug)]
struct Workflow {
    id: String,
    conditions: Vec<Condition>,
    fallback: NextInstruction,
    fallback_value: String,
}
#[derive(Clone, Debug)]
enum Operation {
    GT,
    LT,
}
#[derive(Clone, Debug)]
enum NextInstruction {
    Workflow,
    Reject,
    Accept,
}

#[derive(Clone, Debug)]
struct Condition {
    property: String,
    value: usize,
    operation: Operation,
    next_instruction: NextInstruction,
    next_instruction_value: String,
}

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day(2023, 19, true);
    //let lines: Vec<&str> = input.split("\n").collect();

    let mut workflows: HashMap<String, Workflow> = HashMap::<String, Workflow>::new();
    let mut parts: Vec<Part> = vec![];
    if let Some((all_workflows, all_parts)) = input.split_once("\n\n") {
        parts = all_parts
            .split("\n")
            .map(|part| {
                let split = part
                    .replace("{", "")
                    .replace("}", "")
                    .replace("m=", "")
                    .replace("x=", "")
                    .replace("a=", "")
                    .replace("s=", "")
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                Part {
                    x: split[0],
                    m: split[1],
                    a: split[2],
                    s: split[3],
                }
            })
            .collect::<Vec<Part>>();

        all_workflows.split("\n").for_each(|workflow| {
            let replaced_conditions = workflow.split_once("{").unwrap().1.replace("}", "");
            let mut split_conditions = replaced_conditions.split(",").collect::<Vec<&str>>();
            let (fallback, fallback_value) = match split_conditions.pop() {
                Some("A") => (NextInstruction::Accept, ""),
                Some("R") => (NextInstruction::Reject, ""),
                Some(x) => (NextInstruction::Workflow, x),
                _ => {
                    panic!("bad fallback")
                }
            };

            let mut conditions = vec![];

            for condition in split_conditions {
                let chars: Vec<char> = condition.chars().collect();
                let value = condition
                    .split_once(":")
                    .unwrap()
                    .0
                    .split_at(2)
                    .1
                    .parse::<usize>()
                    .unwrap();
                conditions.push(Condition {
                    property: chars[0].to_string(),
                    operation: match chars[1] {
                        '>' => Operation::GT,
                        '<' => Operation::LT,
                        _ => {
                            panic!("Operation not supported")
                        }
                    },
                    value,
                    next_instruction: match condition.split_once(":").unwrap().1 {
                        "A" => NextInstruction::Accept,
                        "R" => NextInstruction::Reject,
                        _ => NextInstruction::Workflow,
                    },
                    next_instruction_value: condition.split_once(":").unwrap().1.to_string(),
                });
            }
            let mut flow = Workflow {
                id: " ".to_string(),
                conditions,
                fallback,
                fallback_value: fallback_value.to_string(),
            };
            if let Some(name) = workflow.split_once("{") {
                flow.id = name.0.to_string();
            }
            workflows.insert(flow.id.clone(), flow.clone());
        })
    }

    let accepted_parts = parts
        .iter()
        .filter_map(|part| {
            match run_workflow_for_part(part, workflows.get("in").unwrap(), &workflows) {
                true => Some(part),
                false => None,
            }
        })
        .collect::<Vec<&Part>>()
        .iter()
        .fold(0, |acc, part| acc + part.x + part.m + part.a + part.s);
    for part in &parts {
        let workflow = workflows.get("in").unwrap();
        let valid = run_workflow_for_part(part, workflow, &workflows);
        debug!("");
    }

    let answer = accepted_parts;

    println!("Day 19 task 1: {}", answer);
    eprintln!("{:?}", start.elapsed());
}

fn run_workflow_for_part(
    part: &Part,
    workflow: &Workflow,
    workflows: &HashMap<String, Workflow>,
) -> bool {
    let work = workflow.clone();
    let conditions = work.conditions;
    for c in conditions {
        let prop = match c.property.as_str() {
            "x" => part.x,
            "m" => part.m,
            "a" => part.a,
            "s" => part.s,
            _ => {
                panic!("unknown property")
            }
        };
        let value = c.value;
        let satisfied = match c.operation {
            Operation::GT => prop > value,
            Operation::LT => prop < value,
        };
        if !satisfied {
            continue;
        }
        return match c.next_instruction {
            NextInstruction::Accept => true,
            NextInstruction::Reject => false,
            NextInstruction::Workflow => run_workflow_for_part(
                part,
                workflows.get(c.next_instruction_value.as_str()).unwrap(),
                workflows,
            ),
        };
    }

    return match workflow.fallback {
        NextInstruction::Accept => true,
        NextInstruction::Reject => false,
        NextInstruction::Workflow => run_workflow_for_part(
            part,
            workflows.get(work.fallback_value.as_str()).unwrap(),
            workflows,
        ),
    };
}
