use std::collections::{HashSet, VecDeque};

use crate::aoc;

// --- Day 5: Print Queue ---

const EXAMPLE_DATA: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

pub fn main(part: aoc::Part, data: aoc::Data) {
    let data = match data {
        aoc::Data::Web(data) => data,
        aoc::Data::Example => EXAMPLE_DATA.to_string(),
    };
    let result = match part {
        aoc::Part::One => part_one,
        aoc::Part::Two => part_two,
    }(data);

    println!("{}", result);
}

struct PrintOrder {
    depends_on: u32,
    page_no: u32,
}

struct ParsedData {
    ordering: Vec<PrintOrder>,
    queue: Vec<Vec<u32>>,
}

impl ParsedData {
    fn get_page_dependants(&self, page_no: u32) -> Vec<u32> {
        self.ordering
            .iter()
            .filter(|order| order.page_no == page_no)
            .map(|order| order.depends_on)
            .collect()
    }
}

fn parse_data(data: String) -> ParsedData {
    let mut it = data.split("\n\n");

    let ordering = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let depends_on = parts.next().unwrap().parse().unwrap();
            let page_no = parts.next().unwrap().parse().unwrap();
            PrintOrder {
                depends_on,
                page_no,
            }
        })
        .collect();

    let queue = it
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    ParsedData { ordering, queue }
}

fn is_queue_valid(data: &ParsedData, entry: &Vec<u32>) -> bool {
    let mut dependants = HashSet::new();
    for page_no in entry.iter().rev() {
        for depends_on in data.get_page_dependants(*page_no) {
            if entry.contains(&depends_on) {
                if dependants.contains(&depends_on) {
                    return false;
                }
            }
        }
        dependants.insert(*page_no);
    }
    true
}

fn part_one(data: String) -> u32 {
    let data = parse_data(data);
    data.queue
        .iter()
        .filter(|entry| is_queue_valid(&data, entry))
        .map(|entry| entry[entry.len() / 2])
        .sum()
}

fn reorder_queue(data: &ParsedData, pages: &Vec<u32>) -> Vec<u32> {
    let mut queue: VecDeque<_> = pages.iter().rev().collect();
    let mut result = Vec::new();
    'queue: while let Some(page_no) = queue.pop_front() {
        for depends_on in data.get_page_dependants(*page_no) {
            if pages.contains(&depends_on) {
                if !result.contains(&depends_on) {
                    queue.push_back(page_no);
                    continue 'queue;
                }
            }
        }
        result.insert(0, *page_no);
    }
    result
}

fn part_two(data: String) -> u32 {
    let data = parse_data(data);
    data.queue
        .iter()
        .filter(|entry| !is_queue_valid(&data, entry))
        .map(|entry| reorder_queue(&data, entry))
        .map(|entry| entry[entry.len() / 2])
        .sum()
}
