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

#[derive(Debug, PartialEq, Clone)]
struct Side(Vec<Pixel>);

impl Side {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|p| match p {
                Pixel::Illuminated => "1",
                Pixel::NotIlluminated => "0",
            })
            .collect::<Vec<&str>>()
            .concat()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Sides {
    top: Side,
    left: Side,
    bottom: Side,
    right: Side,
}

impl Tile {
    fn sides(&self) -> Sides {
        Sides {
            top: Side(self.pixels[0].iter().cloned().collect()),
            left: Side(
                self.pixels
                    .iter()
                    .map(|row| row[0].clone())
                    .collect::<Vec<Pixel>>(),
            ),
            bottom: Side(self.pixels[self.pixels.len() - 1].iter().cloned().collect()),
            right: Side(
                self.pixels
                    .iter()
                    .map(|row| row[row.len() - 1].clone())
                    .collect::<Vec<Pixel>>(),
            ),
        }
    }
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

fn build_side_map(tiles: &Vec<Tile>) -> HashMap<String, Vec<u64>> {
    let mut m: HashMap<String, Vec<u64>> = HashMap::new();
    for tile in tiles {
        let sides = tile.sides();
        //println!(
        //    "{}: {:?} {:?} {:?} {:?}",
        //    tile.number,
        //    sides.top.to_string(),
        //    sides.left.to_string(),
        //    sides.bottom.to_string(),
        //    sides.right.to_string(),
        //);

        (*m.entry(sides.top.to_string()).or_default()).push(tile.number);
        (*m.entry(sides.left.to_string()).or_default()).push(tile.number);
        (*m.entry(sides.bottom.to_string()).or_default()).push(tile.number);
        (*m.entry(sides.right.to_string()).or_default()).push(tile.number);

        (*m.entry(sides.top.to_string().chars().rev().collect())
            .or_default())
        .push(tile.number);
        (*m.entry(sides.left.to_string().chars().rev().collect())
            .or_default())
        .push(tile.number);
        (*m.entry(sides.bottom.to_string().chars().rev().collect())
            .or_default())
        .push(tile.number);
        (*m.entry(sides.right.to_string().chars().rev().collect())
            .or_default())
        .push(tile.number);
    }

    m
}

fn find_corners(tiles: &Vec<Tile>, m: &HashMap<String, Vec<u64>>) -> Vec<u64> {
    let mut candidates: Vec<u64> = vec![];

    for tile in tiles {
        let sides = tile.sides();

        let sides = vec![
            m.get(&sides.top.to_string()),
            m.get(&sides.left.to_string()),
            m.get(&sides.bottom.to_string()),
            m.get(&sides.right.to_string()),
        ];

        let c = sides
            .iter()
            .map(|k| k.unwrap())
            .filter(|k| k.len() == 1)
            .count();

        if c >= 2 {
            //println!("{}: {:?}", tile.number, sides);
            //println!("{}", tile.number);
            candidates.push(tile.number);
        }

        //println!("{}: {:?}", tile.number, sides);
    }
    candidates
}

fn build_image(
    tiles: &Vec<Tile>,
    side_map: &HashMap<String, Vec<u64>>,
    tile_map: &HashMap<u64, Tile>,

    result: &mut Vec<Vec<u64>>,
    visited: &mut HashSet<u64>,
    start_tile: u64,
    index: (u64, u64),
) {
    if visited.get(&start_tile).is_some() {
        return;
    }

    let tile = &tile_map[&start_tile];
    let sides = tile.sides();

    let sides = vec![
        side_map.get(&sides.top.to_string()),
        side_map.get(&sides.left.to_string()),
        side_map.get(&sides.bottom.to_string()),
        side_map.get(&sides.right.to_string()),
    ];

    visited.insert(start_tile);

    for side in sides {
        for tile_num in side.unwrap() {}
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let tiles = read_input(lines);

    let tile_map: HashMap<u64, Tile> = tiles
        .iter()
        .map(|tile| (tile.number, tile.clone()))
        .collect();

    let m = build_side_map(&tiles);
    let candidates = find_corners(&tiles, &m);

    //println!("{}", &(top_left.unwrap()).number);
    //println!("{}", &(bottom_left.unwrap()).number);
    //println!("{}", &bottom_right.unwrap().number);
    //println!("{}", &top_right.unwrap().number);

    //let res = build_image(&tiles, &m, &tile_map, &mut HashSet::new(), 1951, (0, 0));

    //let heads = tiles.filter(|tile| {});
    println!(
        "{:?}",
        m.values()
            .filter(|v| v.len() == 1)
            .collect::<Vec<&Vec<u64>>>()
    );
    //println!("{}", candidates.iter().fold(1, |acc, a| acc * a));
}
