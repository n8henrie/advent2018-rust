// --- Day 7: The Sum of Its Parts ---
//
// You find yourself standing on a snow-covered coastline; apparently, you landed a little off course. The region is too hilly to see the North Pole from here, but you do spot some Elves that seem to be trying to unpack something that washed ashore. It's quite cold out, so you decide to risk creating a paradox by asking them for directions.
//
// "Oh, are you the search party?" Somehow, you can understand whatever Elves from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the device on your wrist also be a translator? "Those clothes don't look very warm; take this." They hand you a heavy coat.
//
// "We do need to find our way back to the North Pole, but we have higher priorities at the moment. You see, believe it or not, this box contains something that will solve all of Santa's transportation problems - at least, that's what it looks like from the pictures in the instructions." It doesn't seem like they can read whatever language it's in, but you can: "Sleigh kit. Some assembly required."
//
// "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at once!" They start excitedly pulling more parts out of the box.
//
// The instructions specify a series of steps and requirements about which steps must be finished before others can begin (your puzzle input). Each step is designated by a single letter. For example, suppose you have the following instructions:
//
// Step C must be finished before step A can begin.
// Step C must be finished before step F can begin.
// Step A must be finished before step B can begin.
// Step A must be finished before step D can begin.
// Step B must be finished before step E can begin.
// Step D must be finished before step E can begin.
// Step F must be finished before step E can begin.
//
// Visually, these requirements look like this:
//
//
//   -->A--->B--
//  /    \      \
// C      -->D----->E
//  \           /
//   ---->F-----
//
// Your first goal is to determine the order in which the steps should be completed. If more than one step is ready, choose the step which is first alphabetically. In this example, the steps would be completed as follows:
//
//     Only C is available, and so it is done first.
//     Next, both A and F are available. A is first alphabetically, so it is done next.
//     Then, even though F was available earlier, steps B and D are now also available, and B is the first alphabetically of the three.
//     After that, only D and F are available. E is not available because only some of its prerequisites are complete. Therefore, D is completed next.
//     F is the only choice, so it is done next.
//     Finally, E is completed.
//
// So, in this example, the correct order is CABDFE.
//
// In what order should the steps in your instructions be completed?
//
// Part 2:
// Each step takes 60 seconds + different number of seconds to complete (A=1 - Z=26)
// With 5 workers and the 60+ second step durations described above, how long will it take to complete all of the steps?

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
    let btm = parse_input(&input);
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
    let btm = parse_input(&input);
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
    let input = std::fs::read_to_string("input.txt")?;
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
