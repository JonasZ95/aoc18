extern crate chrono;
extern crate time;

#[macro_use]
extern crate nom;

use chrono::TimeZone;
use time::Duration;

use nom::types::CompleteStr;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashMap;
use chrono::Timelike;
use std::iter;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
type Dt = chrono::DateTime<chrono::Utc>;
type SleepTable = HashMap<u64, [usize; 60]>;


fn main() -> Result<()> {
    let r = File::open("in/04.txt")?;
    let r = BufReader::new(r);

    let mut logs: Vec<LogEntry> = Vec::new();
    for line in r.lines() {
        logs.push(line?.parse()?);
    }
    logs.sort_by_key(|l| l.date);

    let mut sleeps: Vec<Sleep> = Vec::new();
    let mut guard = 0;
    let mut start: Option<Dt> = None;

    for log in logs.iter() {
        match log.kind {
            LogKind::GuardBeginsShift(id) => {
                start = None;
                guard = id;
            },
            LogKind::FallsAsleep => {
                if guard == 0 {
                    continue;
                }

                if start.is_none() {
                    start = Some(log.date);
                }
            },
            LogKind::WakesUp => {
                if guard == 0 || start.is_none() {
                    continue;
                }

                let sleep = Sleep::new(start.take().unwrap(), log.date, guard)?;
                sleeps.push(sleep);
            }
        }
    }

    let mut sleep_table_counter =  SleepTable::new();

    for sleep in sleeps.iter() {
        let counter = sleep_table_counter.entry(sleep.guard)
            .or_insert([0; 60]);

        let start = sleep.start.minute();
        let end = sleep.end.minute();

        for i in start..end {
            counter[i as usize] += 1;
        }
    }

    let result1 = part1(&sleeps)?;
    let result2 = part2(&sleeps)?;
    println!("#1: {}, 2: {}", result1, result2);
    Ok(())
}

fn part1(sleeps: &[Sleep]) -> Result<usize> {
    let mut sleep_table: HashMap<u64, Duration> = HashMap::default();
    for sleep in sleeps.iter() {
        let dur = sleep.duration();

           sleep_table.entry(sleep.guard)
               .and_modify(|d| *d = *d + dur )
               .or_insert(dur);
    }

    let max_sleep_guard: u64 = *sleep_table.iter()
        .max_by_key(|(_, v)| v.clone())
        .unwrap()
        .0;

    let mut counter = [0; 60];
    for sleep in sleeps.iter()
        .filter(|s| s.guard == max_sleep_guard) {

        let start = sleep.start.minute();
        let end = sleep.end.minute();

        for i in start..end {
            counter[i as usize] += 1;
        }
    }

    let best_minute = counter
        .iter()
        .enumerate()
        .max_by_key(|(_, v)| v.clone())
        .unwrap()
        .0;

    let result = max_sleep_guard as usize * best_minute;
    Ok(result)
}

fn part2(sleeps: &[Sleep]) -> Result<usize> {
    let mut sleep_table_counter =  HashMap::<u64, [usize; 60]>::new();

    for sleep in sleeps.iter() {
        let counter = sleep_table_counter.entry(sleep.guard)
            .or_insert([0; 60]);

        let start = sleep.start.minute();
        let end = sleep.end.minute();

        for i in start..end {
            counter[i as usize] += 1;
        }
    }

    let it = sleep_table_counter.iter()
        .flat_map(|(&guard, counter)| {
            iter::repeat(guard)
                .zip(counter.iter().enumerate())
        });

    let (guard, (min, count)) = sleep_table_counter.iter()
        .flat_map(|(&guard, counter)| {
            iter::repeat(guard)
                .zip(counter.iter().enumerate())
        })
        .max_by_key(|(_, (_, v))| v.clone())
        .unwrap();

    Ok(guard as usize * min)
}

#[derive(Debug)]
pub struct Sleep {
    guard: u64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
}

impl Sleep {
    fn new(start: Dt, end: Dt, guard: u64) -> Result<Sleep> {
        if start > end {
            return Err(format!("Start data: {} after end date: {}", start, end).into());
        }

        Ok(Sleep {
            guard,
            start,
            end
        })
    }

    fn duration(&self) -> Duration {
        (self.end - self.start)
    }
}

#[derive(Debug)]
enum LogKind {
    FallsAsleep,
    WakesUp,
    GuardBeginsShift(u64),
}

#[derive(Debug)]
pub struct LogEntry {
    date: chrono::DateTime<chrono::Utc>,
    kind: LogKind,
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

impl FromStr for LogEntry {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        log_entry(CompleteStr(s))
            .map(|(_, c)| c)
            .map_err(|e| e.to_string().into())
    }
}


named!(num<CompleteStr, u64>,
       map_res!(take_while!(is_digit), |s: CompleteStr| u64::from_str_radix(s.0, 10))
);


named!(date<CompleteStr, chrono::DateTime<chrono::Utc>>,
       map_res!(take_until!("]"), |s: CompleteStr| chrono::Utc.datetime_from_str(s.0, "%Y-%m-%d %H:%M"))
);


named!(log_kind<CompleteStr, LogKind>, alt!(
    tag!("wakes up") => {|_| LogKind::WakesUp} |
    tag!("falls asleep") => {|_| LogKind::FallsAsleep} |
    do_parse!(
        tag!("Guard #") >>
        id: num >>
        tag!(" begins shift") >>
        (id)
     ) => {|id| LogKind::GuardBeginsShift(id)}
));

named!(log_entry<CompleteStr, LogEntry>,
  do_parse!(
    tag!("[") >>
    date: date >>
    tag!("] ") >>
    kind: log_kind >>

    (LogEntry { date, kind })
  )
);

