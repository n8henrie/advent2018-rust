use std::collections::HashSet;
use std::error;
use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Box::<dyn error::Error>::from(format!($($tt)*)) }
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct System {
    name: String,
    groups: Vec<Group>,
}

#[derive(Debug, PartialEq, Clone)]
struct Group {
    id: usize,
    num_units: usize,
    hit_points: u32,
    attack_damage: u32,
    attack_type: Strategy,
    initiative: u32,
    weaknesses: HashSet<Strategy>,
    immunities: HashSet<Strategy>,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.num_units as u32 * self.attack_damage
    }

    fn get_target_index(&self, targets: &mut Vec<&Group>) -> Result<Option<usize>> {
        let max_damage = targets.iter().fold(<Vec<_>>::new(), |mut v, target| {
            if let Some(&t) = v.first() {
                // Compare amount of the current best (t) vs new candidate (target)
                match self.damage_amount(t).cmp(&self.damage_amount(target)) {
                    std::cmp::Ordering::Greater => v,
                    std::cmp::Ordering::Equal => {
                        v.push(target);
                        v
                    }
                    std::cmp::Ordering::Less => vec![target],
                }
            } else {
                vec![target]
            }
        });
        if let Some(v) = max_damage.first().map(|g| self.damage_amount(g)) {
            if v == 0 {
                // max damage is zero, no point in going further
                return Ok(None);
            }
        }
        if max_damage.len() < 2 {
            let id = max_damage.get(0).map(|t| t.id);
            if let Some(id) = id {
                if let Some(idx) = targets.iter().position(|&g| g.id == id) {
                    targets.remove(idx);
                }
            }
            return Ok(id);
        }
        let largest_effective_power =
            max_damage
                .iter()
                .fold(<Vec<&Group>>::new(), |mut v, target| {
                    if let Some(&t) = v.first() {
                        match t.effective_power().cmp(&target.effective_power()) {
                            std::cmp::Ordering::Greater => v,
                            std::cmp::Ordering::Equal => {
                                v.push(target);
                                v
                            }
                            std::cmp::Ordering::Less => vec![target],
                        }
                    } else {
                        vec![target]
                    }
                });
        if largest_effective_power.len() < 2 {
            let id = largest_effective_power.get(0).map(|t| t.id);
            if let Some(id) = id {
                if let Some(idx) = targets.iter().position(|&g| g.id == id) {
                    targets.remove(idx);
                }
            }
            return Ok(id);
        }
        let highest_initiative =
            largest_effective_power
                .iter()
                .fold(<Vec<&Group>>::new(), |mut v, target| {
                    if let Some(t) = v.first() {
                        match t.initiative.cmp(&target.initiative) {
                            std::cmp::Ordering::Greater => v,
                            std::cmp::Ordering::Equal => {
                                v.push(target);
                                v
                            }
                            std::cmp::Ordering::Less => vec![target],
                        }
                    } else {
                        vec![target]
                    }
                });
        if highest_initiative.len() > 1 {
            return Err(err!("Multiple first-choice targets"));
        }
        let id = highest_initiative.get(0).map(|t| t.id);
        if let Some(id) = id {
            if let Some(idx) = targets.iter().position(|&g| g.id == id) {
                targets.remove(idx);
            }
        }
        Ok(id)
    }

    fn damage_amount(&self, target: &Group) -> u32 {
        self.effective_power()
            * match self.attack_type {
                _ if target.weaknesses.contains(&self.attack_type) => 2,
                _ if target.immunities.contains(&self.attack_type) => 0,
                _ => 1,
            }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Strategy {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

fn parse_strengths_and_weaknesses<'a, 'b>(
    words: &'a mut impl Iterator<Item = &'b str>,
) -> Result<(HashSet<Strategy>, HashSet<Strategy>)> {
    let mut words = words.map(|word| {
        word.trim_start_matches('(')
            .trim_end_matches(')')
            .trim_end_matches(',')
    });
    let no_strengths = words
        .by_ref()
        .next()
        .map(|word| word == "weak")
        .ok_or_else(|| err!("No first word"))?;
    let mut flag = true;
    let first: HashSet<_> = words
        .by_ref()
        .take_while(|word| {
            let oldflag = flag;
            if word.ends_with(';') {
                flag = false;
            }
            oldflag
        })
        .filter_map(|word| word.trim_end_matches(';').parse().ok())
        .collect();
    let second: HashSet<_> = words.filter_map(|word| word.parse().ok()).collect();

    if no_strengths {
        return Ok((second, first));
    }
    Ok((first, second))
}

fn parse_system<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<System> {
    let mut system = Vec::new();
    let name = lines
        .next()
        .ok_or_else(|| err!("No header line"))?
        .to_owned();
    for (idx, line) in lines.enumerate() {
        let mut words = line.split_whitespace();
        let num_units: usize = words
            .next()
            .ok_or_else(|| "did not find num_units")?
            .parse()?;
        ["units", "each", "with"].iter().for_each(|_| {
            words.next();
        });
        let hit_points: u32 = words
            .next()
            .ok_or_else(|| "did not find hit_points")?
            .parse()?;
        ["hit", "points"].iter().for_each(|_| {
            words.next();
        });

        // Wish I could just peek this
        let (immunities, weaknesses) = if let Some(word) = words.next() {
            if word.starts_with('(') {
                let mut flag = false;
                let mut strengths_and_weaknesses = vec![word];
                strengths_and_weaknesses.extend(words.by_ref().take_while(|word| {
                    if flag {
                        return false;
                    }
                    if word.ends_with(')') {
                        flag = true;
                    }
                    true
                }));
                let mut strengths_and_weaknesses = strengths_and_weaknesses.into_iter();
                parse_strengths_and_weaknesses(&mut strengths_and_weaknesses)?
            } else {
                let (a, b) = (HashSet::new(), HashSet::new());
                (a, b)
            }
        } else {
            return Err(err!("No next word"));
        };

        ["an", "attack", "that", "does"].iter().for_each(|_| {
            words.next();
        });
        let attack_damage: u32 = words
            .next()
            .ok_or_else(|| format!("did not find attack_damage around {}: {}", idx, line))?
            .parse()?;
        let attack_type: Strategy = words
            .next()
            .ok_or_else(|| "did not find attack_type")?
            .parse()?;
        ["damage", "at", "initiative"].iter().for_each(|_| {
            words.next();
        });
        let initiative: u32 = words
            .next()
            .ok_or_else(|| "did not find initiative")?
            .parse()?;
        system.push(Group {
            id: idx + 1,
            num_units,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
        });
    }
    Ok(System {
        name,
        groups: system,
    })
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        for group in self.groups.iter() {
            writeln!(f, "Group {} contains {} units", group.id, group.num_units)?
        }
        Ok(())
    }
}

impl FromStr for Strategy {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        use Strategy::*;
        match s.to_lowercase().as_str() {
            "bludgeoning" => Ok(Bludgeoning),
            "cold" => Ok(Cold),
            "fire" => Ok(Fire),
            "radiation" => Ok(Radiation),
            "slashing" => Ok(Slashing),
            _ => Err(err!("Unrecognized strategy")),
        }
    }
}

fn parse_input(input: &str) -> Result<(System, System)> {
    let mut lines = input.lines();
    let immune: System = parse_system(&mut lines.by_ref().take_while(|l| !l.is_empty()))?;
    let infection: System = parse_system(&mut lines)?;
    Ok((immune, infection))
}

fn fight(mut immune: System, mut infection: System) -> Result<(String, u32)> {
    while ![&immune, &infection]
        .iter()
        .any(|system| system.groups.iter().all(|group| group.num_units == 0))
    {
        let mut attack_plan: Vec<(String, usize, u32, usize)> = Vec::new();
        {
            for group in [&mut immune.groups, &mut infection.groups].iter_mut() {
                group.sort_by(|a, b| {
                    a.effective_power()
                        .cmp(&b.effective_power())
                        .then(a.initiative.cmp(&b.initiative))
                });
                group.reverse();
            }
            let pairs = &[(&immune, &infection), (&infection, &immune)];
            for (system, other) in pairs.iter() {
                let mut avail_targets: Vec<_> =
                    other.groups.iter().filter(|g| g.num_units > 0).collect();
                for group in system.groups.iter().filter(|g| g.num_units > 0) {
                    if let Some(target_idx) = group.get_target_index(&mut avail_targets)? {
                        attack_plan.push((
                            system.name.clone(),
                            group.id,
                            group.initiative,
                            target_idx,
                        ));
                    }
                }
            }
        }
        if attack_plan.is_empty() {
            return Err(err!("Stalemate"));
        }
        attack_plan.sort_by_key(|(_, _, initiative, _)| *initiative);
        attack_plan.reverse();
        for (system_name, idx, _, t_idx) in attack_plan.into_iter() {
            let (attacking, target) = match system_name.as_ref() {
                "Immune System:" => (
                    immune
                        .groups
                        .iter()
                        .find(|g| g.id == idx)
                        .ok_or_else(|| "No matching attacking immune group")?,
                    infection
                        .groups
                        .iter_mut()
                        .find(|g| g.id == t_idx)
                        .ok_or_else(|| "No matching target infection group")?,
                ),
                "Infection:" => (
                    infection
                        .groups
                        .iter()
                        .find(|g| g.id == idx)
                        .ok_or_else(|| "No matching attacking infection group")?,
                    immune
                        .groups
                        .iter_mut()
                        .find(|g| g.id == t_idx)
                        .ok_or_else(|| "No matching target immune group")?,
                ),
                _ => return Err(err!("Bad group name")),
            };
            if attacking.num_units == 0 {
                continue;
            }
            let tolerance = target.num_units as u32 * target.hit_points;
            let damage = attacking.damage_amount(target);
            let survived = {
                if damage >= tolerance {
                    0
                } else {
                    let rem = tolerance - damage;
                    let (mut survived, mod_) = (rem / target.hit_points, rem % target.hit_points);
                    if mod_ > 0 {
                        survived += 1
                    }
                    survived as usize
                }
            };
            target.num_units = survived;
        }
    }
    let winner = &[&immune, &infection]
        .iter()
        .find(|s| !s.groups.iter().all(|g| g.num_units == 0))
        .ok_or_else(|| err!("No winner!"))?
        .name;
    Ok((
        winner.to_owned(),
        immune
            .groups
            .iter()
            .chain(infection.groups.iter())
            .map(|group| group.num_units as u32)
            .sum::<u32>(),
    ))
}

fn part1(input: &str) -> Result<u32> {
    let (immune, infection) = parse_input(input)?;
    Ok(fight(immune, infection)?.1)
}

fn part2(input: &str) -> Result<u32> {
    fn boost(system: &mut System, boost: u32) {
        for group in system.groups.iter_mut() {
            group.attack_damage += boost
        }
    }
    let (immune_orig, infection) = parse_input(input)?;

    let (mut minboost, mut maxboost) = (2, 2);
    loop {
        let mut immune = immune_orig.clone();
        boost(&mut immune, maxboost);
        if let Ok((name, _)) = fight(immune, infection.clone()) {
            if name == "Immune System:" {
                break;
            };
        }
        minboost = maxboost;
        maxboost += maxboost;
    }

    let get_mid = |minboost, maxboost| ((maxboost - minboost) / 2) + minboost;
    let mut mid = get_mid(minboost, maxboost);
    loop {
        let mut immune = immune_orig.clone();
        boost(&mut immune, mid);
        if let Ok((name, num_units)) = fight(immune, infection.clone()) {
            if name == "Immune System:" {
                if minboost == mid {
                    return Ok(num_units);
                }
                maxboost = mid;
                mid = get_mid(minboost, mid);
                continue;
            }
        }
        if minboost == mid {
            minboost += 1;
            mid += 1;
            continue;
        }
        minboost = mid;
        mid = get_mid(mid, maxboost);
    }
}

fn main() -> Result<()> {

    let input = std::fs::read_to_string("day24/input.txt")?;
    writeln!(io::stdout(), "part1: {:?}", part1(&input)?)?;
    writeln!(io::stdout(), "part2: {:?}", part2(&input)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_strengths_and_weaknesses() {
        use Strategy::*;
        let testpairs = [
            (
                "(immune to bludgeoning, fire, cold, slashing)",
                (
                    [Bludgeoning, Fire, Cold, Slashing]
                        .iter()
                        .cloned()
                        .collect(),
                    HashSet::new(),
                ),
            ),
            (
                "(immune to slashing, radiation; weak to cold)",
                (
                    [Slashing, Radiation]
                        .iter()
                        .cloned()
                        .collect::<HashSet<_>>(),
                    [Cold].iter().cloned().collect::<HashSet<_>>(),
                ),
            ),
            (
                "(weak to radiation, cold)",
                (
                    <HashSet<Strategy>>::new(),
                    [Radiation, Cold].iter().cloned().collect::<HashSet<_>>(),
                ),
            ),
        ];
        for line in testpairs.iter() {
            assert_eq!(
                parse_strengths_and_weaknesses(&mut line.0.split_whitespace()).unwrap(),
                line.1
            );
        }
    }

    #[test]
    fn test_parse() -> Result<()> {
        let input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an \
attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, \
slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that \
does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, \
cold) with an attack that does 12 slashing damage at initiative 4";
        let (immune, infection) = parse_input(input)?;
        assert_eq!(
            immune.to_string().trim(),
            "Immune System:
Group 1 contains 17 units
Group 2 contains 989 units"
        );
        assert_eq!(
            infection.to_string().trim(),
            "Infection:
Group 1 contains 801 units
Group 2 contains 4485 units"
        );
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an \
attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, \
slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that \
does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, \
cold) with an attack that does 12 slashing damage at initiative 4";
        let (immune, infection) = parse_input(input)?;
        assert_eq!(fight(immune, infection)?.1, 5216);
        Ok(())
    }
}
