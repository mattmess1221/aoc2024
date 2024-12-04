use crate::aoc;

mod day1;

pub async fn run_day(day: u32, part: aoc::Part, data_source: aoc::DataSource) {
    match day {
        1 => day1::main(part, data_source).await,
        _ => panic!("Day {} not implemented", day),
    }
}