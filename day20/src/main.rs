// --- Day 20: A Regular Map ---
//
// While you were learning about instruction pointers, the Elves made
// considerable progress. When you look up, you discover that the North Pole
// base construction project has completely surrounded you.
//
// The area you are in is made up entirely of rooms and doors. The rooms are
// arranged in a grid, and rooms only connect to adjacent rooms when a door is
// present between them.
//
// For example, drawing rooms as ., walls as #, doors as | or -, your current
// position as X, and where north is up, the area you're in might look like
// this:
//
// #####
// #.|.#
// #-###
// #.|X#
// #####
//
// You get the attention of a passing construction Elf and ask for a map. "I
// don't have time to draw out a map of this place - it's huge. Instead, I can
// give you directions to every room in the facility!" He writes down some
// directions on a piece of parchment and runs off. In the example above, the
// instructions might have been ^WNE$, a regular expression or "regex" (your
// puzzle input).
//
// The regex matches routes (like WNE for "west, north, east") that will take
// you from your current room through various doors in the facility. In
// aggregate, the routes will take you through every door in the facility at
// least once; mapping out all of these routes will let you build a proper map
// and find your way around.
//
// ^ and $ are at the beginning and end of your regex; these just mean that the
// regex doesn't match anything outside the routes it describes. (Specifically,
// ^ matches the start of the route, and $ matches the end of it.) These
// characters will not appear elsewhere in the regex.
//
// The rest of the regex matches various sequences of the characters N (north),
// S (south), E (east), and W (west). In the example above, ^WNE$ matches only
// one route, WNE, which means you can move west, then north, then east from
// your current position. Sequences of letters like this always match that exact
// route in the same order.
//
// Sometimes, the route can branch. A branch is given by a list of options
// separated by pipes (|) and wrapped in parentheses. So, ^N(E|W)N$ contains a
// branch: after going north, you must choose to go either east or west before
// finishing your route by going north again. By tracing out the possible routes
// after branching, you can determine where the doors are and, therefore, where
// the rooms are in the facility.
//
// For example, consider this regex: ^ENWWW(NEEE|SSE(EE|N))$
//
// This regex begins with ENWWW, which means that from your current position,
// all routes must begin by moving east, north, and then west three times, in
// that order. After this, there is a branch. Before you consider the branch,
// this is what you know about the map so far, with doors you aren't sure about
// marked with a ?:
//
// #?#?#?#?#
// ?.|.|.|.?
// #?#?#?#-#
//     ?X|.?
//     #?#?#
//
// After this point, there is (NEEE|SSE(EE|N)). This gives you exactly two
// options: NEEE and SSE(EE|N). By following NEEE, the map now looks like this:
//
// #?#?#?#?#
// ?.|.|.|.?
// #-#?#?#?#
// ?.|.|.|.?
// #?#?#?#-#
//     ?X|.?
//     #?#?#
//
// Now, only SSE(EE|N) remains. Because it is in the same parenthesized group as
// NEEE, it starts from the same room NEEE started in. It states that starting
// from that point, there exist doors which will allow you to move south twice,
// then east; this ends up at another branch. After that, you can either move
// east twice or north once. This information fills in the rest of the doors:
//
// #?#?#?#?#
// ?.|.|.|.?
// #-#?#?#?#
// ?.|.|.|.?
// #-#?#?#-#
// ?.?.?X|.?
// #-#-#?#?#
// ?.|.|.|.?
// #?#?#?#?#
//
// Once you've followed all possible routes, you know the remaining unknown
// parts are all walls, producing a finished map of the facility:
//
// #########
// #.|.|.|.#
// #-#######
// #.|.|.|.#
// #-#####-#
// #.#.#X|.#
// #-#-#####
// #.|.|.|.#
// #########
//
// Sometimes, a list of options can have an empty option, like (NEWS|WNSE|).
// This means that routes at this point could effectively skip the options in
// parentheses and move on immediately. For example, consider this regex and the
// corresponding map:
//
// ^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$
//
// ###########
// #.|.#.|.#.#
// #-###-#-#-#
// #.|.|.#.#.#
// #-#####-#-#
// #.#.#X|.#.#
// #-#-#####-#
// #.#.|.|.|.#
// #-###-###-#
// #.|.|.#.|.#
// ###########
//
// This regex has one main route which, at three locations, can optionally
// include additional detours and be valid: (NEWS|), (WNSE|), and (SWEN|).
// Regardless of which option is taken, the route continues from the position it
// is left at after taking those steps. So, for example, this regex matches all
// of the following routes (and more that aren't listed here):
//
//     ENNWSWWSSSEENEENNN
//     ENNWSWWNEWSSSSEENEENNN
//     ENNWSWWNEWSSSSEENEESWENNNN
//     ENNWSWWSSSEENWNSEEENNN
//By following the various routes the regex matches, a full map of all of the
//doors and rooms in the facility can be assembled.
//
//To get a sense for the size of this facility, you'd like to determine which
//room is furthest from you: specifically, you would like to find the room for
//which the shortest path to that room would require passing through the most
//doors.
//
//     In the first example (^WNE$), this would be the north-east corner 3 doors
//     away. In the second example (^ENWWW(NEEE|SSE(EE|N))$), this would be the
//     south-east corner 10 doors away. In the third example
//     (^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$), this would be the north-east
//     corner 18 doors away.
//
// Here are a few more examples:
//
// Regex: ^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$
// Furthest room requires passing 23 doors
//
// #############
// #.|.|.|.|.|.#
// #-#####-###-#
// #.#.|.#.#.#.#
// #-#-###-#-#-#
// #.#.#.|.#.|.#
// #-#-#-#####-#
// #.#.#.#X|.#.#
// #-#-#-###-#-#
// #.|.#.|.#.#.#
// ###-#-###-#-#
// #.|.#.|.|.#.#
// #############
//
// Regex: ^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$

// Example: ^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS)))N)$
// WSSEESWWWNW
// S  |     NENNEEEENN
//      ESSSSW      |    WSWWN
//    NWSW | SSEN        E | WWS
//                           E | SS

// Furthest room requires passing 31 doors
//
// ###############
// #.|.|.|.#.|.|.#
// #-###-###-#-#-#
// #.|.#.|.|.#.#.#
// #-#########-#-#
// #.#.|.|.|.|.#.#
// #-#-#########-#
// #.#.#.|X#.|.#.#
// ###-#-###-#-#-#
// #.|.#.#.|.#.|.#
// #-###-#####-###
// #.|.#.|.|.#.#.#
// #-#-#####-#-#-#
// #.#.|.|.|.#.|.#
// ###############
//
// What is the largest number of doors you would be required to pass through to
// reach a room? That is, find the room for which the shortest path from your
// starting location to that room would require passing through the most doors;
// what is the fewest doors you can pass through to reach it?
//
// part1: 4778

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
        let paths = parse_regex(&input)
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
    fn longest(&self) -> u32 {
        *self.grid.values().map(|(_, count)| count).max().unwrap()
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
            vec![Rc::clone(&node)]
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
                paths.extend(recurse(&n, history.clone()));
            }
            paths.sort();
            paths.dedup();
            paths
        }
        recurse(node, Vec::new())
    }

    fn longest(node: &Rc<RefCell<Node>>) -> usize {
        fn recurse(node: &Rc<RefCell<Node>>, steps: usize) -> Vec<usize> {
            let len = node.borrow().directions.len();
            if node.borrow().next.is_empty() {
                vec![steps + len]
            } else {
                let mut paths = Vec::new();
                for n in node.borrow().next.iter() {
                    paths.extend(recurse(n, steps + len))
                }
                paths
            }
        }
        recurse(node, 0).into_iter().max().unwrap()
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
    let hs: HashSet<NodeHasher> = HashSet::from_iter(iter.into_iter().map(|node| NodeHasher(node)));
    hs.into_iter().map(|n| n.0).collect::<Vec<_>>()
}

#[derive(Eq, PartialEq)]
struct NodeHasher(Rc<RefCell<Node>>);
impl Hash for NodeHasher {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
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
                    while let Some(c) = iter.next() {
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
            // _ => panic!(format!("bad character: {}", c)),
            _ => unreachable!(),
        }
    }
}

fn part1(input: &str) -> u32 {
    let area = Area::from_regex(input);
    // println!("{}", area.to_string());
    area.longest()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let input = input.trim();

    println!("part1: {}", part1(&input));
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
