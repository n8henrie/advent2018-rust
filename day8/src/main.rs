use nom::{alt, count, digit, do_parse, eof, eol, map_res, named, space, types::CompleteStr};

#[derive(Debug)]
struct Node {
    header: (usize, usize),
    children: Vec<Node>,
    metadata: Vec<u32>,
}

named!(parse_input<CompleteStr, Node>,
       do_parse!(
           num_children: map_res!(digit, |CompleteStr(n)| n.parse()) >>
           space >>
           num_metadata: map_res!(digit, |CompleteStr(n)| n.parse()) >>
           space >>
           children: count!(parse_input, num_children) >>
           metadata: count!(do_parse!(
                   num: map_res!(digit, |CompleteStr(n)| n.parse()) >>
                   alt!(space | eof!() | eol) >>
                   ( num )
           ), num_metadata) >>
           ( Node { header: (num_children, num_metadata), children, metadata } )
       )
);

fn metadata_sum(node: &Node) -> u32 {
    node.metadata.iter().sum::<u32>() + node.children.iter().map(|c| metadata_sum(c)).sum::<u32>()
}

fn part1(input: &str) -> u32 {
    let node = match parse_input(CompleteStr(input)) {
        Ok((_incomplete, node)) => node,
        Err(e) => panic!("{}", e.to_string()),
    };
    metadata_sum(&node)
}

fn root_sum(node: &Node) -> u32 {
    if let 0 = node.children.len() {
        node.metadata.iter().sum::<u32>()
    } else {
        node.metadata
            .iter()
            .map(|m| {
                let idx = (m - 1) as usize;
                if let Some(c) = node.children.get(idx) {
                    root_sum(c)
                } else {
                    0
                }
            })
            .sum::<u32>()
    }
}

fn part2(input: &str) -> u32 {
    let node = match parse_input(CompleteStr(input)) {
        Ok((_incomplete, node)) => node,
        Err(e) => panic!("{}", e),
    };
    root_sum(&node)
}

fn main() -> std::io::Result<()> {

    let input = std::fs::read_to_string("day8/input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(part1(test_input), 138);
    }

    #[test]
    fn test_part2() {
        let test_input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(part2(test_input), 66);
    }
}
