use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::io::{self, BufRead, Error, Lines, Split, StdinLock};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Pixel {
    Illuminated,
    NotIlluminated,
}

impl FromStr for Pixel {
    type Err = io::Error;

    fn from_str(st: &str) -> Result<Self, Self::Err> {
        match st {
            "." => Ok(Pixel::NotIlluminated),
            "#" => Ok(Pixel::Illuminated),
            _ => Err(Error::new(
                io::ErrorKind::Other,
                format!("could not parse {}", st),
            )),
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Pixel::*;

        match &self {
            Illuminated => write!(f, "#"),
            NotIlluminated => write!(f, "."),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
    number: u64,
    pixels: Vec<Vec<Pixel>>,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let st: String = self
            .pixels
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| format!("{}", cell))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", st)
    }
}

fn read_input(lines: Lines<StdinLock>) -> Vec<Tile> {
    let tile_re = Regex::new(r"Tile (?P<num>\d+):").unwrap();

    let mut peekable = lines.map(|li| li.unwrap()).peekable();
    let mut tiles = vec![];

    while peekable.peek().is_some().to_owned() {
        let t = peekable.by_ref().nth(0).unwrap();
        let caps = tile_re.captures(&t).unwrap();
        let pixels: Vec<Vec<Pixel>> = peekable
            .by_ref()
            .take_while(|li| li != "")
            .map(|li| {
                li.chars()
                    .map(|c| c.to_string().parse::<Pixel>().unwrap())
                    .collect()
            })
            .collect();

        let tile = Tile {
            number: caps["num"].parse().unwrap(),
            pixels: pixels,
        };

        tiles.push(tile);
    }
    tiles
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let tiles = read_input(lines);

    println!("{}", tiles[0]);
}
