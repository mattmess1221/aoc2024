mod aoc;
mod days;

use aoc::Part;
use clap::Parser;
use dotenv;

#[derive(Parser, Debug)]
struct Args {
    day: u32,

    #[arg(short, long, default_value = "1")]
    part: Part,

    #[arg(short, long, default_value_t = false)]
    live: bool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let args = Args::parse();
    // days::run_day(1, Part::One, DataSource::Web).await;
    days::run_day(args.day, args.part, args.live).await;
}
