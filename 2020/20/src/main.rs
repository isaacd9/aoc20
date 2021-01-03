use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::io::{self, BufRead, Error, Lines, Split, StdinLock};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(PartialEq, Clone)]
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

impl fmt::Debug for Pixel {
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

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
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
    fn flip(&mut self) {}

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

static DIFFS: &'static [(i64, i64)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Image<'_> {
    fn find_complimentary_tiles(
        &self,
        used: &HashSet<u64>,
        surrounding_tile_nos: Vec<u64>,
    ) -> Vec<u64> {
        let mut surrounding_tiles = surrounding_tile_nos.iter().map(|s| &self.tile_map[&s]);

        let first_tile = surrounding_tiles.nth(0);
        if first_tile.is_none() {
            return vec![];
        }
        //println!("first tile: {:?}", surrounding_tile_nos[0]);

        let mut matched_tiles: Vec<u64> = vec![];
        for side in first_tile.unwrap().sides().0.iter() {
            for k in vec![&side.1.to_string()] {
                let maybe_matched_tile = self.side_map[k]
                    .iter()
                    .filter(|(_direction, tile_no)| *tile_no != surrounding_tile_nos[0])
                    .nth(0);
                //println!("checking tile {:?}, {:?}", maybe_matched_tile, used);

                if maybe_matched_tile.is_none() {
                    continue;
                }

                let matched_tile = maybe_matched_tile.unwrap().1;

                if used.contains(&matched_tile) {
                    continue;
                }

                matched_tiles.push(matched_tile);
            }
        }

        //println!("matched: {:?}", matched_tiles);
        matched_tiles
            .iter()
            .filter(|matched_tile| {
                let sides = self.tile_map[&matched_tile].sides();

                let possible_tiles: Vec<u64> = sides
                    .0
                    .iter()
                    .flat_map(|side| self.side_map[&side.1.to_string()].iter().map(|(_, no)| no))
                    .copied()
                    .collect::<Vec<u64>>();

                for surrounding_tile in surrounding_tile_nos.iter() {
                    if !possible_tiles.contains(surrounding_tile) {
                        return false;
                    }
                }

                true
            })
            .copied()
            .collect()
    }

    fn rec_build_image(
        &self,
        board: &Vec<Vec<u64>>,
        coords: (usize, usize),
        used: &mut HashSet<u64>,
    ) -> Option<Vec<Vec<u64>>> {
        let mut new_board = board.clone();

        // Last row
        if coords.0 >= board.len() {
            return Some(new_board);
        }

        //println!("{:?}", board);
        // Move forward a col or down a row
        let new_coords = if coords.1 == board[coords.0].len() - 1 {
            (coords.0 + 1, 0)
        } else {
            (coords.0, coords.1 + 1)
        };

        let surrounding_tiles: Vec<u64> = DIFFS
            .iter()
            .map(|d| {
                board
                    .get((d.0).wrapping_add(coords.0 as i64) as usize)
                    .and_then(|a| a.get(d.1.wrapping_add(coords.1 as i64) as usize))
            })
            .filter(|a| a.is_some())
            .map(|a| a.unwrap())
            .filter(|a| **a != 0)
            .copied()
            .collect();

        //println!("surrounding: {:?} {:?}", coords, surrounding_tiles);
        let comps = self.find_complimentary_tiles(&used, surrounding_tiles);

        for comp in comps {
            new_board[coords.0][coords.1] = comp;
            let mut new_used = used.clone();
            new_used.insert(comp);
            match self.rec_build_image(&new_board, (new_coords.0, new_coords.1), &mut new_used) {
                Some(board) => return Some(board),
                None => continue,
            }
        }

        None
    }

    fn find_ids(&self, corners: &Vec<u64>) -> Option<Vec<Vec<u64>>> {
        use Direction::*;
        let size = (self.tiles.len() as f64).sqrt() as usize;
        let mut board = vec![vec![0; size]; size];

        for first_corner_no in corners {
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

            match self.rec_build_image(&board.clone(), (0, 1), &mut used.clone()) {
                Some(board) => return Some(board),
                None => continue,
            }
        }
        None
    }

    fn render(&self, ids: &Vec<Vec<u64>>) -> Vec<Vec<Pixel>> {
        use crate::Pixel::*;

        let t = &self.tile_map[&ids[0][0]];
        let grid_size = (self.tiles.len() as f64).sqrt() as usize;
        let size = grid_size * t.pixels.len();
        let mut tiles = vec![vec![NotIlluminated; size]; size];

        for (row_i, row) in ids.iter().enumerate() {
            for (cell_i, cell) in row.iter().enumerate() {
                let surrounding_tiles: Vec<(u64, Direction)> = DIFFS
                    .iter()
                    .map(|d| {
                        let direction = match d {
                            (1, 0) => Direction::Bottom,
                            (0, 1) => Direction::Right,
                            (-1, 0) => Direction::Bottom,
                            (0, -1) => Direction::Top,
                            _ => panic!("impossible direction"),
                        };
                        (
                            ids.get((d.0).wrapping_add(row_i as i64) as usize)
                                .and_then(|a| a.get(d.1.wrapping_add(cell_i as i64) as usize)),
                            direction,
                        )
                    })
                    .filter(|a| a.0.is_some())
                    .map(|a| (a.0.unwrap().clone(), a.1))
                    .collect();

                let sides = self.tile_map[cell].sides();
                let comps: HashMap<_, _> = sides
                    .0
                    .iter()
                    .map(|side| &self.side_map[&side.1.to_string()])
                    .flat_map(|comps| comps.iter())
                    .filter(|comp| {
                        surrounding_tiles
                            .iter()
                            .map(|a| a.0)
                            .position(|a| a == comp.1)
                            .is_some()
                    })
                    .map(|comp| {
                        (
                            comp.1,
                            self.tile_map[&comp.1]
                                .sides()
                                .0
                                .iter()
                                .filter(move |side| side.0 == comp.0)
                                .map(|side| side.1.to_string())
                                .collect::<Vec<String>>()[0]
                                .clone(),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                let sides_to_direction = surrounding_tiles
                    .iter()
                    .map(|item| (item.1.clone(), comps[&item.0].clone()))
                    .collect::<HashMap<_, _>>();

                println!(
                    "{:?} {:?}",
                    sides_to_direction,
                    sides
                        .0
                        .iter()
                        .map(|(direction, side)| (direction.clone(), side.to_string()))
                        .collect::<Vec<(Direction, String)>>()
                )
            }
        }

        tiles
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
    //let corners = find_corners(&tiles, &side_map);
    let corners = vec![1951, 3079, 2971, 1171];

    //println!("{:?}", m.values().map(|v| v.len() 1).collect::<Vec<_>>());
    println!("{:?}", corners);
    //println!("{:?}", corners.iter().fold(1, |acc, corner| acc * corner));

    //println!("{:?}", tiles.len());

    let img = Image {
        tiles: &tiles,
        side_map: &side_map,
        tile_map: &tile_map,
    };
    let ids = img.find_ids(&corners);
    for row in &ids {
        for cell in row {
            println!("{:?}", cell);
        }
    }
    let result = img.render(&ids.unwrap());
    for row in result {
        println!("{:?}", row);
    }
}
