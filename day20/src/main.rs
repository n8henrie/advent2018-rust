use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::rc::Rc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Direction {
    N,
    S,
    E,
    W,
}

struct Area {
    grid: HashMap<(usize, usize), (Gridpoint, u32)>,
    xmax: usize,
    ymax: usize,
}

impl Area {
    fn from_regex(input: &str) -> Self {
        use Direction::*;
        use Gridpoint::*;
        let mut hm: HashMap<(isize, isize), (Gridpoint, u32)> = HashMap::new();
        let mut cursor: (isize, isize) = (0, 0);
        hm.insert(cursor, (Start, 0));
        let paths = parse_regex(input)
            .iter()
            .flat_map(Node::follow)
            .collect::<Vec<_>>();

        for path in paths {
            let mut stepcount = 0;
            for step in path {
                match step {
                    N => {
                        cursor.1 += 1;
                        hm.insert(cursor, (NSDoor, 0));
                        cursor.1 += 1;
                        if let Some((_, count)) = hm.get(&cursor) {
                            stepcount = *count;
                        } else {
                            stepcount += 1;
                            hm.insert(cursor, (Room, stepcount));
                        }
                    }
                    E => {
                        cursor.0 += 1;
                        hm.insert(cursor, (WEDoor, 0));
                        cursor.0 += 1;
                        if let Some((_, count)) = hm.get(&cursor) {
                            stepcount = *count;
                        } else {
                            stepcount += 1;
                            hm.insert(cursor, (Room, stepcount));
                        }
                    }
                    S => {
                        cursor.1 -= 1;
                        hm.insert(cursor, (NSDoor, 0));
                        cursor.1 -= 1;
                        if let Some((_, count)) = hm.get(&cursor) {
                            stepcount = *count;
                        } else {
                            stepcount += 1;
                            hm.insert(cursor, (Room, stepcount));
                        }
                    }
                    W => {
                        cursor.0 -= 1;
                        hm.insert(cursor, (WEDoor, 0));
                        cursor.0 -= 1;
                        if let Some((_, count)) = hm.get(&cursor) {
                            stepcount = *count;
                        } else {
                            stepcount += 1;
                            hm.insert(cursor, (Room, stepcount));
                        }
                    }
                }
            }
            cursor = (0, 0);
        }
        let (xs, ys): (Vec<_>, Vec<_>) = hm.keys().cloned().unzip();
        let (xmin, xmax, ymin, ymax) = (
            xs.iter().min().unwrap(),
            xs.iter().max().unwrap(),
            ys.iter().min().unwrap(),
            ys.iter().max().unwrap(),
        );
        let mut adjusted_hm: HashMap<(usize, usize), (Gridpoint, u32)> = HashMap::new();

        // Add 1 to account for the wall border
        for ((x, y), g) in hm {
            adjusted_hm.insert(
                (
                    (x - xmin + 1).try_into().unwrap(),
                    (y - ymin + 1).try_into().unwrap(),
                ),
                g,
            );
        }
        // Add 3 to account for inner and outer wall borders
        Area {
            grid: adjusted_hm,
            xmax: (xmax - xmin + 3).try_into().unwrap(),
            ymax: (ymax - ymin + 3).try_into().unwrap(),
        }
    }
}

impl fmt::Display for Gridpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Gridpoint::*;
        write!(
            f,
            "{}",
            match self {
                Start => 'X',
                NSDoor => '-',
                WEDoor => '|',
                Room => '.',
                Wall => '#',
            }
        )
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.ymax {
            for x in 0..self.xmax {
                write!(
                    f,
                    "{}",
                    self.grid
                        // ymax - y to invert the y axis, since we are printing
                        // from the top down, -1 to account for wall
                        .get(&(x, self.ymax - y - 1))
                        .map(|(room, _)| room)
                        .unwrap_or(&Gridpoint::default())
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Node {
    fn new(directions: impl Into<String>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            directions: directions.into(),
            ..Default::default()
        }))
    }
    fn tails(node: &Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
        if node.borrow().next.is_empty() {
            vec![Rc::clone(node)]
        } else {
            let mut tails = Vec::new();
            for n in node.borrow().next.iter() {
                tails.extend(Node::tails(n));
            }
            dedup_rc(tails.into_iter())
        }
    }

    fn follow(node: &Rc<RefCell<Node>>) -> Vec<Vec<Direction>> {
        fn recurse(node: &Rc<RefCell<Node>>, history: Vec<Direction>) -> Vec<Vec<Direction>> {
            let mut history = history;
            let dirs: Vec<Direction> = node
                .borrow()
                .directions
                .chars()
                .map(Direction::from)
                .collect();
            history.extend(dirs);

            if node.borrow().next.is_empty() {
                return vec![history];
            } else {
            }
            let mut paths: Vec<Vec<Direction>> = Vec::new();
            for n in node.borrow().next.iter() {
                paths.extend(recurse(n, history.clone()));
            }
            paths.sort();
            paths.dedup();
            paths
        }
        recurse(node, Vec::new())
    }
}

enum Gridpoint {
    Start,
    NSDoor,
    WEDoor,
    Room,
    Wall,
}

impl Default for Gridpoint {
    fn default() -> Self {
        Gridpoint::Wall
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Node {
    directions: String,
    next: Vec<Rc<RefCell<Node>>>,
}

fn dedup_rc<T>(iter: T) -> Vec<Rc<RefCell<Node>>>
where
    T: Iterator<Item = Rc<RefCell<Node>>>,
{
    let hs: HashSet<NodeHasher> = HashSet::from_iter(iter.map(NodeHasher));
    hs.into_iter().map(|n| n.0).collect::<Vec<_>>()
}

#[derive(Eq)]
struct NodeHasher(Rc<RefCell<Node>>);
impl Hash for NodeHasher {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}
impl PartialEq for NodeHasher {
    fn eq(&self, other: &NodeHasher) -> bool {
        self.0.as_ptr().eq(&other.0.as_ptr())
    }
}

fn parse_regex(input: &str) -> Vec<Rc<RefCell<Node>>> {
    fn parse(input: impl Into<String>) -> Vec<Rc<RefCell<Node>>> {
        let input = input.into();
        let mut iter = input.chars();
        let mut head = String::new();
        while let Some(c) = iter.next() {
            match c {
                '(' if head.is_empty() => {
                    let mut opts = Vec::new();
                    let mut depth = 0;
                    for c in &mut iter {
                        match c {
                            '|' if depth == 0 => {
                                for node in parse(head.drain(..).collect::<String>()) {
                                    opts.push(node)
                                }
                            }
                            '(' => {
                                head.push(c);
                                depth += 1;
                            }
                            ')' => {
                                if depth == 0 {
                                    for node in parse(head.drain(..).collect::<String>()) {
                                        opts.push(node)
                                    }
                                    break;
                                }
                                head.push(c);
                                depth -= 1;
                            }
                            _ => head.push(c),
                        }
                    }
                    let tail = parse(iter.collect::<String>());
                    let tails = opts.iter().map(Node::tails).flatten().collect::<Vec<_>>();
                    for node in tails.iter() {
                        node.borrow_mut().next = tail.clone();
                    }
                    return opts;
                }
                '(' => {
                    let headnodes = parse(head);
                    let mut tail = c.to_string();
                    tail.extend(iter);
                    let next = parse(tail);

                    let tails = headnodes.iter().flat_map(Node::tails).collect::<Vec<_>>();
                    for node in tails.iter() {
                        node.borrow_mut().next = next.clone();
                    }
                    return headnodes;
                }
                '^' | '$' => (),
                _ => head.push(c),
            }
        }
        if head.is_empty() {
            return vec![];
        }
        vec![Node::new(head)]
    }
    parse(input)
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'N' => Direction::N,
            'E' => Direction::E,
            'S' => Direction::S,
            'W' => Direction::W,
            _ => unreachable!(),
        }
    }
}

fn part1(input: &str) -> u32 {
    let area = Area::from_regex(input);
    *area.grid.values().map(|(_, count)| count).max().unwrap()
}

fn part2(input: &str) -> usize {
    let area = Area::from_regex(input);
    area.grid
        .values()
        .filter(|(_, count)| *count >= 1000)
        .count()
}

fn main() -> Result<()> {

    let input = std::fs::read_to_string("day20/input.txt")?;
    let input = input.trim();

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let input = "^ENWWW(NEEE|SSE(EE|N))$";
        let solution = "#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########
";
        let output = Area::from_regex(input).to_string();
        assert_eq!(solution, output);

        let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        let solution = "###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########
";
        let output = Area::from_regex(input).to_string();
        assert_eq!(solution, output);

        let input = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        let solution = "#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############
";
        let output = Area::from_regex(input).to_string();
        assert_eq!(solution, output);

        let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let solution = "###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############
";
        let output = Area::from_regex(input).to_string();
        assert_eq!(solution, output);
    }

    #[test]
    fn test_parse() {
        let input = "^NN((SS|EE)NNN|EEE(SSS|WW|WWW))WWWW$";
        let nn = Node::new("NN");
        let ss = Node::new("SS");
        let ee = Node::new("EE");
        let nnn = Node::new("NNN");
        let eee = Node::new("EEE");
        let sss = Node::new("SSS");
        let ww = Node::new("WW");
        let www = Node::new("WWW");
        let wwww = Node::new("WWWW");
        nn.borrow_mut().next = vec![Rc::clone(&ss), Rc::clone(&ee), Rc::clone(&eee)];
        ss.borrow_mut().next = vec![Rc::clone(&nnn)];
        ee.borrow_mut().next = vec![Rc::clone(&nnn)];
        nnn.borrow_mut().next = vec![Rc::clone(&wwww)];
        eee.borrow_mut().next = vec![Rc::clone(&sss), Rc::clone(&ww), Rc::clone(&www)];
        sss.borrow_mut().next = vec![Rc::clone(&wwww)];
        ww.borrow_mut().next = vec![Rc::clone(&wwww)];
        www.borrow_mut().next = vec![Rc::clone(&wwww)];

        let output = parse_regex(input);
        assert_eq!(output, vec![nn]);
    }

    #[test]
    fn test_follow() {
        use Direction::*;
        let input = "^NN(SS|EE)WW$";
        let nodes = parse_regex(input);
        let solution = vec![vec![N, N, S, S, W, W], vec![N, N, E, E, W, W]];

        let output = nodes.iter().flat_map(Node::follow).collect::<Vec<_>>();
        assert_eq!(solution, output);
    }

    #[test]
    fn test_part1() {
        let input = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        let output = part1(input);
        assert_eq!(23, output);

        let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let output = part1(input);
        assert_eq!(31, output);
    }
}
