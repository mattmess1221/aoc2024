use regex::Regex;

use crate::aoc;

const EXAMPLE_DATA_1: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const EXAMPLE_DATA_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

pub fn main(part: aoc::Part, data: aoc::Data) {
    let data = match data {
        aoc::Data::Web(data) => data,
        aoc::Data::Example => match part {
            aoc::Part::One => EXAMPLE_DATA_1.to_string(),
            aoc::Part::Two => EXAMPLE_DATA_2.to_string(),
        },
    };

    let result = match part {
        aoc::Part::One => part_one,
        aoc::Part::Two => part_two,
    }(data);

    println!("{}", result);
}

struct Prog {
    total: u32,
    enabled: bool,
}

impl Prog {
    fn new() -> Self {
        Self {
            total: 0,
            enabled: true,
        }
    }
    fn mul(&mut self, a: u32, b: u32) {
        if self.enabled {
            self.total += a * b
        }
    }
}
fn part_one(data: String) -> u32 {
    let re = Regex::new(r"mul\((?P<a>\d+),(?P<b>\d+)\)").unwrap();
    let mut prog = Prog::new();

    for cap in re.captures_iter(data.as_str()) {
        let a = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let b = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        prog.mul(a, b);
    }
    prog.total
}

fn part_two(data: String) -> u32 {
    let re =
        Regex::new(r"(?P<mul>mul)\((?P<a>\d+),(?P<b>\d+)\)|(?P<do>do)\(\)|(?P<dont>don't)\(\)")
            .unwrap();
    let mut prog = Prog::new();
    for cap in re.captures_iter(data.as_str()) {
        if cap.name("mul").is_some() {
            let a = cap.name("a").unwrap().as_str().parse::<u32>().unwrap();
            let b = cap.name("b").unwrap().as_str().parse::<u32>().unwrap();
            prog.mul(a, b);
        } else if cap.name("do").is_some() {
            prog.enabled = true;
        } else if cap.name("dont").is_some() {
            prog.enabled = false;
        }
    }
    prog.total
}
