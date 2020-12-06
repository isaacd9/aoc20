use std::cmp::Ordering;
use std::io::{self, BufRead, Error, Lines, StdinLock};

#[derive(Default, Debug, Eq, PartialEq)]
struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.row.cmp(&other.row).then(self.col.cmp(&other.col))
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.row.cmp(&other.row).then(self.col.cmp(&other.col)))
    }
}

fn bin_search(
    code: &mut dyn Iterator<Item = char>,
    range: (u32, u32),
    sigils: (char, char),
) -> u32 {
    let mut bottom = range.0;
    let mut top = range.1;

    for c in code {
        match c {
            k if k == sigils.0 => top -= (top - bottom) / 2 + 1,
            k if k == sigils.1 => bottom += (top - bottom - 1) / 2 + 1,
            _ => panic!("unexpected arg: {}", c),
        }
        //println!("bottom: {}, top: {}", bottom, top)
    }

    assert_eq!(bottom, top);
    bottom
}

fn parse_seat(st: String) -> Seat {
    let mut seat: Seat = Default::default();

    let mut iter = st.chars();
    let mut row_code = iter.by_ref().take(7);

    seat.row = bin_search(&mut row_code, (0, 128), ('F', 'B'));

    let mut col_code = iter.by_ref().take(3);
    seat.col = bin_search(&mut col_code, (0, 7), ('L', 'R'));

    seat
}

fn find_seats(lines: Lines<StdinLock>) -> Result<Vec<Seat>, io::Error> {
    let seats = lines
        .map(|line| {
            let li = line.unwrap();
            parse_seat(li)
        })
        .collect();
    Ok(seats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seat() {
        let seat = parse_seat("FBFBBFFRLR".to_string());
        assert_eq!(seat.row, 44);
        assert_eq!(seat.col, 5);
        assert_eq!(seat.id(), 357);

        let seat = parse_seat("BFFFBBFRRR".to_string());
        assert_eq!(seat.row, 70);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.id(), 567);

        let seat = parse_seat("FFFBBBFRRR".to_string());
        assert_eq!(seat.row, 14);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.id(), 119);

        let seat = parse_seat("BBFFBBFRLL".to_string());
        assert_eq!(seat.row, 102);
        assert_eq!(seat.col, 4);
        assert_eq!(seat.id(), 820);
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut seats = find_seats(lines).unwrap();

    let mut max = 0;
    for seat in &seats {
        let seat_id = seat.id();
        if seat_id > max {
            max = seat_id;
        }
    }
    println!("{}", max);

    seats.sort();

    let mut cur_row = seats[0].row;
    let mut cur_col = seats[0].col - 1;

    for seat in &seats {
        if seat.col == cur_col + 1 {
            cur_col = seat.col;
        } else if seat.row == cur_row + 1 {
            // Next row, reset
            cur_row = seat.row;
            cur_col = seat.col;
        } else {
            break;
        }
    }

    println!(
        "{}",
        Seat {
            row: cur_row,
            col: cur_col + 1
        }
        .id()
    )
}
