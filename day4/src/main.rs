use nom::types::CompleteStr;
use nom::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq)]
enum GuardEvent {
    BeginsShift { dt: DateTime, id: u32 },
    FallsAsleep { dt: DateTime },
    WakesUp { dt: DateTime },
}

impl Ord for GuardEvent {
    fn cmp(&self, other: &GuardEvent) -> Ordering {
        let lhs = match &self {
            GuardEvent::BeginsShift { dt, .. } => dt,
            GuardEvent::FallsAsleep { dt } => dt,
            GuardEvent::WakesUp { dt } => dt,
        };
        let rhs = match other {
            GuardEvent::BeginsShift { dt, .. } => dt,
            GuardEvent::FallsAsleep { dt } => dt,
            GuardEvent::WakesUp { dt } => dt,
        };
        lhs.cmp(rhs)
    }
}

impl PartialOrd for GuardEvent {
    fn partial_cmp(&self, other: &GuardEvent) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Time {
    hour: u8,
    minute: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DateTime {
    date: Date,
    time: Time,
}

named!(parse_date<CompleteStr, Date>,
       do_parse!(
            year: map_res!(take!(4), |CompleteStr(s)| u16::from_str_radix(s, 10)) >>
            char!('-') >>
            month: map_res!(take!(2), |CompleteStr(s)| u8::from_str_radix(s, 10)) >>
            char!('-') >>
            day: map_res!(take!(2), |CompleteStr(s)| u8::from_str_radix(s, 10)) >>
            ( Date{year, month,day} )
            )
       );

named!(parse_time<CompleteStr, Time>,
       do_parse!(
            hour: map_res!(take!(2), |CompleteStr(s)| u8::from_str_radix(s, 10)) >>
            char!(':') >>
            minute: map_res!(take!(2), |CompleteStr(s)| u8::from_str_radix(s, 10)) >>
            ( Time{hour, minute} )
           )
       );

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(begins_shift<CompleteStr, u32>,
                         do_parse!(
            tag!("Guard") >>
            space >>
            char!('#') >>
            id: map_res!(take_while1!(is_digit), |CompleteStr(s)| u32::from_str_radix(s, 10)) >>
            space >>
            tag!("begins shift") >>
            ( id )
                             )
                         );

named!(parse_datetime<CompleteStr, DateTime>,
       do_parse!(
            date: parse_date >>
            space >>
            time: parse_time >>
            ( DateTime { date, time } )
            )
       );

// [1518-06-12 23:57] Guard #2633 begins shift
named!(parse_line<CompleteStr, GuardEvent>,
       do_parse!(
            dt: delimited!(char!('['), parse_datetime, char!(']')) >>
            space >>
            event: alt!(
                map!(begins_shift, |id| GuardEvent::BeginsShift{dt: dt.clone(), id}) |
                map!(tag!("falls asleep"), |_| GuardEvent::FallsAsleep{dt: dt.clone()}) |
                map!(tag!("wakes up"), |_| GuardEvent::WakesUp{dt: dt.clone()})
                     ) >>

            ( event )
       )
       );

named!(parse_lines<CompleteStr, Vec<GuardEvent>>,
       separated_list_complete!(
           char!('\n'),
           parse_line
           )
       );

fn accumulate_events(events: &[GuardEvent]) -> HashMap<u32, Vec<u32>> {
    let mut sleep_times = HashMap::new();
    let mut latest_id: Option<u32> = None;
    for window in events.windows(2) {
        match window {
            [GuardEvent::BeginsShift { id, .. }, _] => {
                latest_id = Some(*id);
            }
            [GuardEvent::FallsAsleep { dt: sleeps }, GuardEvent::WakesUp { dt: wakes }] => {
                let times = sleep_times
                    .entry(latest_id.unwrap())
                    .or_insert_with(|| vec![0_u32; 60]);
                for time in sleeps.time.minute..wakes.time.minute {
                    times[time as usize] += 1;
                }
            }
            _ => (),
        }
    }
    sleep_times
}

/// Takes HashMap of id: vec<minutes asleep>, where vec is 60 minutes long,
/// and returns the (id, vec) with the most minutes asleep
fn find_sleepiest(sleep_times: &HashMap<u32, Vec<u32>>) -> (&u32, &Vec<u32>) {
    sleep_times
        .iter()
        .max_by_key(|(_, v)| v.iter().sum::<u32>())
        .unwrap()
}

fn find_most_asleep_minute(times: &[u32]) -> u8 {
    times.iter().enumerate().max_by_key(|(_, n)| *n).unwrap().0 as u8
}

fn part1(sleep_times: &HashMap<u32, Vec<u32>>) -> u32 {
    let (id, times) = find_sleepiest(&sleep_times);
    let minute = find_most_asleep_minute(&times);
    u32::from(minute) * id
}

fn part2(sleep_times: &HashMap<u32, Vec<u32>>) -> u32 {
    let sleepiest = sleep_times
        .iter()
        .map(|(id, mins)| {
            (
                id,
                mins.iter().enumerate().max_by_key(|(_idx, &v)| v).unwrap(),
            )
        })
        .max_by_key(|(_id, (_minute, &val))| val)
        .unwrap();
    *sleepiest.0 * (sleepiest.1).0 as u32
}

fn main() -> io::Result<()> {

    let input = fs::read_to_string("day4/input.txt")?;

    let mut events = match parse_lines(CompleteStr(&input)) {
        Ok((_remaining, value)) => value,
        Err(e) => panic!("Parse error: {}", e),
    };
    events.sort_unstable();
    let sleep_times = accumulate_events(&events);

    println!("part 1: {:?}", part1(&sleep_times));
    println!("part 2: {:?}", part2(&sleep_times));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_sleepiest() {
        let sleep_times: HashMap<u32, Vec<u32>> =
            [(3, vec![0, 0, 0]), (4, vec![1, 4, 0]), (19, vec![1, 1, 1])]
                .iter()
                .cloned()
                .collect();
        assert_eq!(find_sleepiest(&sleep_times), (&4, &vec![1, 4, 0]));
    }

    #[test]
    fn test_find_most_asleep_minute() {
        assert_eq!(find_most_asleep_minute(&[0, 0, 0, 1, 0, 4, 2, 0]), 5)
    }
}
