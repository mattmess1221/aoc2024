use crate::aoc;

const EXAMPLE_DATA: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
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

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Self> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
            Direction::SouthWest,
            Direction::SouthEast,
        ]
        .iter()
        .copied()
    }

    fn vector(&self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::NorthWest => (-1, -1),
            Self::NorthEast => (1, -1),
            Self::SouthWest => (-1, 1),
            Self::SouthEast => (1, 1),
        }
    }
}

struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    fn slice<'a>(&self, x: i32, y: i32, d: Direction, len: usize) -> Option<Vec<&T>> {
        let (dx, dy) = d.vector();
        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            let nx = x + dx * i as i32;
            let ny = y + dy * i as i32;
            if let Some(c) = self.get(nx as usize, ny as usize) {
                result.push(c);
            } else {
                return None;
            }
        }
        Some(result)
    }

    fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.data.len()).flat_map(|y| (0..self.data[y].len()).map(move |x| (x, y)))
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect())
    }
}

fn parse_grid(data: String) -> Grid<char> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn vec_vec_slice<'a>(
    grid: &Grid<char>,
    x: i32,
    y: i32,
    d: Direction,
    len: usize,
) -> Option<Vec<char>> {
    let (dx, dy) = d.vector();
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        let nx = x + dx * i as i32;
        let ny = y + dy * i as i32;
        if let Some(c) = grid.get(nx as usize, ny as usize) {
            result.push(*c);
        } else {
            return None;
        }
    }
    Some(result)
}

type WordPredicate = fn(&Grid<char>, i32, i32) -> usize;

fn count_words(grid: Grid<char>, predicate: WordPredicate) -> usize {
    grid.coords()
        .map(|(x, y)| predicate(&grid, x as i32, y as i32))
        .sum()
}

fn is_xmas(grid: &Grid<char>, x: i32, y: i32) -> usize {
    Direction::iter()
        .filter_map(|dir| grid.slice(x, y, dir, 4))
        .filter(|w| String::from_iter(w.clone()) == "XMAS")
        .count()
}

fn part_one(data: String) -> usize {
    let grid = parse_grid(data);
    count_words(grid, is_xmas)
}

fn is_x_mas(grid: &Grid<char>, x: i32, y: i32) -> usize {
    let w1 = vec_vec_slice(&grid, x - 1, y - 1, Direction::SouthEast, 3);
    let w2 = vec_vec_slice(&grid, x + 1, y - 1, Direction::SouthWest, 3);

    match w1.zip(w2) {
        Some((w1, w2)) => {
            let w1 = String::from_iter(w1);
            let w2 = String::from_iter(w2);
            if (w1 == "MAS" || w1 == "SAM") && (w2 == "MAS" || w2 == "SAM") {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn part_two(data: String) -> usize {
    let grid = parse_grid(data);
    count_words(grid, is_x_mas)
}
