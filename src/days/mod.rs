use crate::aoc;

mod day1;
mod day2;
mod day3;
mod day4;

pub async fn run_day(day: u32, part: aoc::Part, live: bool) {
    let func = match day {
        1 => day1::main,
        2 => day2::main,
        3 => day3::main,
        4 => day4::main,
        _ => panic!("Day {} not implemented", day),
    };

    let data = if live {
        aoc::Data::Web(aoc::get_input(day).await)
    } else {
        aoc::Data::Example
    };

    func(part, data);
}
