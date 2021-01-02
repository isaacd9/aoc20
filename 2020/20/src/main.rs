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
enum Direction {
    Top,
    Left,
    Bottom,
    Right,
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
struct Sides(Vec<(Direction, Side)>);

impl Tile {
    fn sides(&self) -> Sides {
        use crate::Direction::*;

        Sides(vec![
            (Top, Side(self.pixels[0].iter().cloned().collect())),
            (
                Left,
                Side(
                    self.pixels
                        .iter()
                        .map(|row| row[0].clone())
                        .collect::<Vec<Pixel>>(),
                ),
            ),
            (
                Bottom,
                Side(self.pixels[self.pixels.len() - 1].iter().cloned().collect()),
            ),
            (
                Right,
                Side(
                    self.pixels
                        .iter()
                        .map(|row| row[row.len() - 1].clone())
                        .collect::<Vec<Pixel>>(),
                ),
            ),
        ])
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

fn build_side_map(tiles: &Vec<Tile>) -> HashMap<String, Vec<(Direction, u64)>> {
    let mut m: HashMap<String, Vec<(Direction, u64)>> = HashMap::new();
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

        for side in sides.0 {
            (*m.entry(side.1.to_string()).or_default()).push((side.0.clone(), tile.number));
            (*m.entry(side.1.to_string().chars().rev().collect())
                .or_default())
            .push((side.0, tile.number));
        }
    }

    m
}

fn find_corners(tiles: &Vec<Tile>, m: &HashMap<String, Vec<(Direction, u64)>>) -> Vec<u64> {
    let mut candidates: Vec<u64> = vec![];

    let edges = m
        .iter()
        .filter(|(_, v)| v.len() < 2)
        .flat_map(|(_, v)| v.iter())
        .collect::<Vec<&(Direction, u64)>>();

    let mut by_piece: HashMap<u64, Vec<&(Direction, u64)>> = HashMap::new();

    for edge in &edges {
        (*by_piece.entry(edge.1).or_default()).push(edge);
    }

    by_piece
        .iter()
        .filter(|(k, v)| v.len() > 2)
        .map(|(k, _)| k)
        .copied()
        .collect()
}

struct Image<'a> {
    tiles: &'a Vec<Tile>,
    side_map: &'a HashMap<String, Vec<(Direction, u64)>>,
    tile_map: &'a HashMap<u64, Tile>,
}

impl Image<'_> {
    fn find_complimentary_tile(&self, used: &HashSet<u64>, in_tile_no: u64) -> u64 {
        let left_tile = &self.tile_map[&in_tile_no];
        let mut r = 0;

        for side in left_tile.sides().0.iter() {
            let maybe_matched_tile = self.side_map[&side.1.to_string()]
                .iter()
                .filter(|(_direction, tile_no)| *tile_no != in_tile_no)
                .nth(0);

            if maybe_matched_tile.is_none() {
                continue;
            }

            let matched_tile = maybe_matched_tile.unwrap().1;

            if used.contains(&matched_tile) {
                continue;
            }

            return matched_tile;
        }

        0
    }

    fn build_image(&self, corners: &Vec<u64>) -> Vec<Vec<u64>> {
        use Direction::*;
        let size = (self.tiles.len() as f64).sqrt() as usize;
        let mut board = vec![vec![0; size]; size];

        let first_corner_no = &corners[0];
        let first_corner = &self.tile_map[first_corner_no];

        let first_side = first_corner
            .sides()
            .0
            .iter()
            .filter(|dir_side| {
                let matches = &self.side_map[&dir_side.1.to_string()];
                matches.len() > 1
            })
            .map(|side| side.1.to_string())
            .nth(0);

        let mut used: HashSet<u64> = HashSet::new();

        board[0][0] = *first_corner_no;
        used.insert(*first_corner_no);

        for row in 0..size {
            if row != 0 {
                let above_tile_no = board[row - 1][0];

                let comp = self.find_complimentary_tile(&used, above_tile_no);

                board[row][0] = comp;
                used.insert(comp);
            }
            for col in 1..size {
                let left_tile_no = board[row][col - 1];

                let comp = self.find_complimentary_tile(&used, left_tile_no);

                board[row][col] = comp;
                used.insert(comp);

                println!("{:?}", board)
            }
        }

        board
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

    let side_map = build_side_map(&tiles);
    let corners = find_corners(&tiles, &side_map);

    //println!("{:?}", m.values().map(|v| v.len() 1).collect::<Vec<_>>());
    println!("{:?}", corners);
    //println!("{:?}", corners.iter().fold(1, |acc, corner| acc * corner));

    //println!("{:?}", tiles.len());

    let result = Image {
        tiles: &tiles,
        side_map: &side_map,
        tile_map: &tile_map,
    }
    .build_image(&corners);
    println!("{:?}", result);
}
