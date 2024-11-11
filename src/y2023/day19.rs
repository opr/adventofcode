use std::arch::aarch64::float32x2_t;
use std::cmp;
use crate::input_reader::input_reader::read_input_for_day;
use log::{debug, error};
use std::collections::{HashMap, HashSet};
use std::panic::panic_any;

#[derive(Clone, Debug)]
struct PartRanges {
    x_lower: usize,
    x_upper: usize,
    m_lower: usize,
    m_upper: usize,
    a_lower: usize,
    a_upper: usize,
    s_lower: usize,
    s_upper: usize,
}

#[derive(Clone,Debug)]
struct RangeResult {
    part_ranges: HashMap<String, (usize,usize)>,
    pass: bool,
    from: String,
    total: usize
}
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
    let mut input = read_input_for_day(2023, 19, false);
    //let lines: Vec<&str> = input.split("\n").collect();
    run_task( input );
}

fn run_task( input: String ) -> usize {
    let start = std::time::Instant::now();
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

    if let Some( first_workflow ) = workflows.iter().find( | (name, _) | return name == &"in" ) {
        let ( id, workflow ) = first_workflow;
        let mut part_sizes: HashMap<String,(usize,usize)> = HashMap::new();

        part_sizes.insert( "x".to_string(), ( 1, 4000 ) );
        part_sizes.insert( "m".to_string(), ( 1, 4000 ) );
        part_sizes.insert( "a".to_string(), ( 1, 4000 ) );
        part_sizes.insert( "s".to_string(), ( 1, 4000 ) );

        let mut results: Vec<RangeResult> = Vec::new();
        let ggg = wf( workflow, part_sizes, &workflows );
        let p2answer = ggg.iter().fold( 0, |acc, part| {
            if ! part.pass {
                return acc;
            }

            return acc + calculate_total( part.part_ranges.clone() )
        } );
        //println!("Part 1: {}", id);
        println!("Part 2: {}", p2answer);
        return p2answer;
    }
    return 0;
}

fn calculate_total( part_ranges: HashMap<String, (usize,usize)> ) -> usize {
    let ( x_range_lower, x_range_upper ) = part_ranges.get( "x" ).unwrap();
    let ( m_range_lower, m_range_upper)  = part_ranges.get( "m" ).unwrap();
    let ( a_range_lower, a_range_upper)  = part_ranges.get( "a" ).unwrap();
    let ( s_range_lower, s_range_upper)  = part_ranges.get( "s" ).unwrap();

    ( ( x_range_upper - x_range_lower ) + 1 ) * ( ( m_range_upper - m_range_lower ) + 1 ) * ( ( a_range_upper - a_range_lower ) + 1 ) * ( ( s_range_upper - s_range_lower ) + 1 )
}

fn wf (workflow: &Workflow, mut part_ranges: HashMap<String, (usize,usize)>,
       workflows: &HashMap<String, Workflow> ) -> Vec< RangeResult > {
    let condition = workflow.conditions.clone();
    let mut results: Vec<RangeResult> = Vec::new();

    for c in condition {

        let ( current_range_lower, current_range_upper ) = part_ranges.get( c.property.as_str() ).unwrap();
        let ( pass_range_start, pass_range_end, fail_range_start, fail_range_end ): ( usize, usize, usize, usize ) = match c.operation {
            Operation::GT => (
                cmp::max( *current_range_lower, c.value + 1 ), *current_range_upper,
                *current_range_lower, cmp::min(c.value, *current_range_upper ),
            ),
            Operation::LT => (
                *current_range_lower, cmp::min(c.value - 1, *current_range_upper ),
                cmp::max( *current_range_lower, c.value ), *current_range_upper,
            ),
        };
        let mut pass_ranges = part_ranges.clone();
        let prop = c.property;
        pass_ranges.insert( prop.clone(), (pass_range_start, pass_range_end) );
        let mut fail_ranges = part_ranges.clone();
        fail_ranges.insert( prop.clone(), (fail_range_start, fail_range_end) );

        match c.next_instruction {
            NextInstruction::Accept => {
                part_ranges = pass_ranges.clone();
                results.push( RangeResult {
                    part_ranges,
                    pass: true,
                    from: workflow.id.clone(),
                    total: calculate_total( pass_ranges.clone() ),
                } );
            },
            NextInstruction::Workflow => {
                let pass_result = wf( workflows.get( c.next_instruction_value.as_str() ).unwrap(), pass_ranges.clone(), workflows );
                results = [ results.clone(), pass_result ].concat();
            }
            _ => {}
        };
        part_ranges = fail_ranges.clone();
    }

    match workflow.fallback {
        NextInstruction::Accept => {
            results.push( RangeResult {
                part_ranges: part_ranges.clone(),
                pass: true,
                from: workflow.id.clone(),
                total: calculate_total( part_ranges.clone() ),
            } );
        },
        NextInstruction::Workflow => {
            let fallback_result = wf( workflows.get( workflow.fallback_value.as_str() ).unwrap(), part_ranges.clone(), workflows );
            results = [ results.clone(), fallback_result ].concat();
        },
        _ => {}
    };
    results
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p2_test1() {
        let input = "in{a>2000:A,R}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),128000000000000)
    }
    #[test]
    fn p2_test2() {
        let input = "in{a>2000:A,b}
b{s<10:A,R}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),128319840000000)
    }
    #[test]
    fn p2_test3() {
        let input = "in{a>2000:A,b}
b{s<10:c,R}
c{x>3000:A,R}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),128079960000000)
    }
    #[test]
    fn p2_test4() {
        let input = "in{a>2000:A,b}
b{s<10:c,R}
c{a>300:A,R}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),128271840000000)
    }
    #[test]
    fn p2_test5() {
        let input = "px{a<2006:qkq,R}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,R}
crn{x>2662:A,R}
in{s<1351:px,R}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),15320205000000)
    }
    #[test]
    fn p2_test6() {
        let input = "px{a<2006:qkq,R}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,R}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),29806731000000)
    }
    #[test]
    fn p2_test7() {
        let input = "px{a<2006:qkq,m>2090:A,R}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,R}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),50372847000000)
    }
    #[test]
    fn p2_test8() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,R}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),58632578689662)
    }
    #[test]
    fn p2_test9() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,R}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),93960578689662)
    }
    #[test]
    fn p2_test10() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),137277746689662)
    }
    #[test]
    fn p2_test11() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,R}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),159103602689662)
    }
    #[test]
    fn p2_test12() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),167245503449662)
    }
    #[test]
    fn p2_test13() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(run_task(input.to_string()),167245503449662)
    }
    #[test]
    fn p2_testfinal() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(run_task(input.to_string()),167409079868000)
    }
}
