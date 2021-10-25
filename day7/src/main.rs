use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

type TaskNode = Rc<RefCell<Task>>;

#[derive(Debug, Clone)]
struct Task {
    name: char,
    depends_on: Vec<Rc<RefCell<Self>>>,
    status: Status,
}

impl Task {
    fn new(name: char) -> Self {
        Task {
            name,
            depends_on: Vec::new(),
            status: Status::Pending,
        }
    }
    fn new_node(name: char) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(name)))
    }
}

#[derive(Debug, Clone)]
enum Status {
    Pending,
    InProcess,
    Done,
}

#[derive(Debug, Clone)]
enum Worker {
    Available,
    WorkingOn((TaskNode, u32)),
}

fn parse_input(input: &str) -> BTreeMap<char, TaskNode> {
    let input = input.to_uppercase();
    let mut tasks = BTreeMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        if let (Some(depends), Some(name)) = (words.nth(1), words.nth(5)) {
            let depends = depends.parse::<char>().expect("unable to parse as char");
            let depends = Rc::clone(
                tasks
                    .entry(depends)
                    .or_insert_with(|| Task::new_node(depends)),
            );

            let name = name.parse::<char>().expect("unable to parse as char");
            let task = tasks.entry(name).or_insert_with(|| Task::new_node(name));
            task.borrow_mut().depends_on.push(depends);
        }
    }
    tasks
}

fn next_task(btm: &BTreeMap<char, TaskNode>) -> Option<(&char, &TaskNode)> {
    btm.iter().find(|(_k, v)| {
        let v = v.borrow();
        (match v.status {
            Status::Pending => true,
            _ => false,
        }) && (v.depends_on.iter().all(|t| match t.borrow().status {
            Status::Done => true,
            _ => false,
        }))
    })
}

fn part1(input: &str) -> String {
    let btm = parse_input(input);
    let mut output = String::new();
    while let Some((name, task)) = next_task(&btm) {
        task.borrow_mut().status = Status::Done;
        output.push(*name);
    }
    output
}

fn name_to_delay(c: char) -> u8 {
    c as u8 - 64
}

fn part2(input: &str, num_workers: u32, delay: u32) -> u32 {
    let btm = parse_input(input);
    let mut workers = vec![Worker::Available; num_workers as usize];
    let mut second = 0;
    loop {
        // First iterate through to complete all tasks that have finished
        for worker in workers.iter_mut() {
            if let Worker::WorkingOn((task, done_at)) = worker {
                if *done_at == second {
                    task.borrow_mut().status = Status::Done;
                    *worker = Worker::Available;
                }
            }
        }

        // Start work on any now ublocked tasks
        for worker in workers.iter_mut() {
            if let Worker::Available = worker {
                if let Some((name, task)) = next_task(&btm) {
                    task.borrow_mut().status = Status::InProcess;
                    let delay = delay + u32::from(name_to_delay(*name));
                    *worker = Worker::WorkingOn((task.clone(), second + delay));
                }
            }
        }

        if workers.iter().all(|w| match w {
            Worker::Available => true,
            _ => false,
        }) {
            return second;
        } else {
            second += 1;
        }
    }
}

fn main() -> std::io::Result<()> {

    let input = std::fs::read_to_string("day7/input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input, 5, 60));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";
        assert_eq!(part1(test_input), "CABDFE");
    }

    #[test]
    fn test_part2() {
        let test_input = "
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";
        assert_eq!(part2(test_input, 2, 0), 15);
    }

    #[test]
    fn test_name_to_delay() {
        assert_eq!(name_to_delay('A'), 1);
        assert_eq!(name_to_delay('Z'), 26);
    }
}
