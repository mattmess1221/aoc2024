use crate::aoc;

const EXAMPLE_DATA: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
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

fn parse_data(data: String) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

fn is_data_safe(data: &Vec<u32>) -> bool {
    let mut it = data.iter();
    let mut prev = it.next().unwrap();
    let mut dir = None;
    for next in it {
        let diff = prev.abs_diff(*next);
        if diff < 1 || diff > 3 {
            return false;
        }
        match &dir {
            None => {
                dir = Some(if prev < next {
                    Direction::Ascending
                } else {
                    Direction::Descending
                })
            }
            Some(dir) => {
                if *dir == Direction::Ascending && prev > next
                    || *dir == Direction::Descending && prev < next
                {
                    return false;
                }
            }
        }
        prev = next;
    }

    return true;
}

fn is_data_safe_with_tolerance(data: &Vec<u32>) -> bool {
    if is_data_safe(&data) {
        return true;
    }
    let mut idx_to_remove = 0;
    while idx_to_remove < data.len() {
        let mut tolerable = data.clone();
        tolerable.remove(idx_to_remove);
        idx_to_remove += 1;

        if is_data_safe(&tolerable) {
            println!("Removed idx {} from {:?}", idx_to_remove, &data);
            return true;
        }
    }

    false
}

fn part_one(data: String) -> usize {
    let data = parse_data(data);
    data.iter().filter(|x| is_data_safe(x)).count()
}

fn part_two(data: String) -> usize {
    let data = parse_data(data);
    data.iter()
        .filter(|x| is_data_safe_with_tolerance(x))
        .count()
}
