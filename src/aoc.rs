use std::{error::Error, fmt::Display};

use clap::ValueEnum;
use reqwest;

async fn read_cache(day: u32) -> Option<String> {
    tokio::fs::read_to_string(format!("cache/day{}.txt", day))
        .await
        .ok()
}

async fn write_cache(day: u32, data: &str) -> Result<(), Box<dyn Error>> {
    tokio::fs::create_dir("cache").await?;
    tokio::fs::write(format!("cache/day{}.txt", day), data).await?;
    tokio::fs::write("cache/.gitignore", "*").await?;
    Ok(())
}

async fn load_input_from_web(day: u32) -> String {
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let session = std::env::var("AOC_SESSION").expect("AOC_SESSION not set");

    reqwest::Client::new()
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .await
        .expect("Failed to send request")
        .text()
        .await
        .expect("Failed to get response text")
}

pub async fn get_input(day: u32) -> String {
    match read_cache(day).await {
        Some(data) => data,
        None => {
            let data = load_input_from_web(day).await;
            write_cache(day, &data)
                .await
                .expect("Failed to write cache");
            data
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Part {
    #[clap(name = "1")]
    One,
    #[clap(name = "2")]
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => write!(f, "Part 1"),
            Part::Two => write!(f, "Part 2"),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DataSource {
    Web,
    Example,
}
