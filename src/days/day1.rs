use std::{collections::HashMap, iter::zip};

use crate::aoc;

const EXAMPLE_DATA: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
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

fn parse_data(data: String) -> (Vec<u32>, Vec<u32>) {
    data.lines()
        .map(|line| {
            let mut it = line.split("   ").map(|x| x.parse::<u32>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .unzip()
}

fn part_one(data: String) -> u32 {
    let (mut a, mut b) = parse_data(data);

    a.sort_by(|x, y| x.partial_cmp(y).unwrap());
    b.sort_by(|x, y| x.partial_cmp(y).unwrap());

    zip(a, b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn counting<T: IntoIterator<Item=u32>>(vec: T) -> HashMap<u32, u32> {
    let mut counts: HashMap<u32, u32> = HashMap::new();

    for i in vec.into_iter() {
        let count = counts.entry(i.clone()).or_insert(0);
        *count += 1;
    }

    counts
}

fn part_two(data: String) -> u32 {
    let (a, b) = parse_data(data);

    let b_counts = counting(b);

    a.iter().map(|x| x * b_counts.get(x).unwrap_or(&0)).sum()
}
