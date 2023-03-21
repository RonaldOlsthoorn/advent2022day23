
use std::{io::{BufReader, BufRead}, fs::File, collections::VecDeque};

const BACKSPACE: char = 8u8 as char;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Heading {
    North = 0,
    South = 1,
    West = 2,
    East = 3
}

use std::convert::TryFrom;

impl TryFrom<i32> for Heading {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Heading::North),
            1 => Ok(Heading::South),
            2 => Ok(Heading::West),
            3 => Ok(Heading::East),
            _ => Err(()),
        }
    }
}

impl Heading {

    fn tick(self: &Self) -> Self {
        return (*self as i32 + 1).rem_euclid(4).try_into().unwrap();
    }
}

fn spread(mut field: VecDeque<VecDeque<bool>>) -> usize {

    let mut rounds = 0;

    let mut first_heading = Heading::North;

    pad_field(&mut field);

    loop {
        println!("i: {}", rounds);

        rounds += 1;

        //print_field(&field);

        let moves = determine_moves(&field, &first_heading);
        move_all(&mut field, &moves);

        pad_field(&mut field);

        if check_distance(&field) {
            break;
        }

        first_heading = first_heading.tick();
    }

    println!("number of rounds: {}", rounds);

    println!("print final: ");
    print_field(&field);

    return rounds;
}

fn determine_moves(field: &VecDeque<VecDeque<bool>>, first_heading: &Heading) -> Vec<(usize, usize, usize, usize)> {

    let mut moves: Vec<(usize, usize, usize, usize)> = Vec::new();

    for (row_index, row) in field.iter().enumerate() {
        for (col_index, tile) in row.iter().enumerate() {

            if *tile {

                if !field[row_index - 1][col_index - 1]
                && !field[row_index - 1][col_index]
                && !field[row_index - 1][col_index + 1]
                && !field[row_index][col_index - 1]
                && !field[row_index][col_index + 1]                
                && !field[row_index + 1][col_index - 1]
                && !field[row_index + 1][col_index]
                && !field[row_index + 1][col_index + 1] {
                    continue;
                }

                let mut heading = *first_heading;

                for _ in 0..4 {
                    match heading {
                        Heading::North => {
                            if field[row_index - 1][col_index - 1]
                            || field[row_index - 1][col_index]
                            || field[row_index - 1][col_index + 1] {
                                heading = heading.tick();
                            } else {
                                moves.push((row_index, col_index, row_index - 1, col_index));
                                break;
                            }
                        },
                        Heading::West => {
                            if field[row_index - 1][col_index - 1]
                            || field[row_index][col_index - 1]
                            || field[row_index + 1][col_index - 1] {
                                heading = heading.tick();
                            } else {
                                moves.push((row_index, col_index, row_index, col_index - 1));
                                break;    
                            }
                        },
                        Heading::South => {
                            if field[row_index + 1][col_index - 1]
                            || field[row_index + 1][col_index]
                            || field[row_index + 1][col_index + 1] {
                                heading = heading.tick();
                            } else {
                                moves.push((row_index, col_index, row_index + 1, col_index));
                                break;
                            }
                        },
                        Heading::East => {
                            if field[row_index - 1][col_index + 1]
                            || field[row_index][col_index + 1]
                            || field[row_index + 1][col_index + 1] {
                                heading = heading.tick();
                            } else {
                                moves.push((row_index, col_index, row_index, col_index + 1));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    moves

}

fn move_all(field: &mut VecDeque<VecDeque<bool>>, moves: &Vec<(usize, usize, usize, usize)>) {

    for (row_index, col_index, new_row, new_col) in moves.iter() {
        
        if moves.iter().filter(|e| &e.2==new_row && &e.3==new_col).count() == 1 {
            field[*row_index][*col_index] = false;
            field[*new_row][*new_col] = true;
        }
    }
} 

fn check_distance(field: &VecDeque<VecDeque<bool>>) -> bool {

    for (row_index, row) in field.iter().enumerate() {
        for (col_index, element) in row.iter().enumerate() {
            if *element && (
            field[row_index - 1][col_index - 1]
            || field[row_index - 1][col_index]
            || field[row_index - 1][col_index + 1]
            || field[row_index][col_index - 1]
            || field[row_index][col_index + 1]                
            || field[row_index + 1][col_index - 1]
            || field[row_index + 1][col_index]
            || field[row_index + 1][col_index + 1]) {
                return false;
            } 
        }
    }

    true
}

fn calculate_box(field: &VecDeque<VecDeque<bool>>) -> usize {

    let mut max_row = 0;
    let mut max_col = 0;
    let mut min_row = field.len();
    let mut min_col = field[0].len();
    let mut count_elves = 0;

    for (row_index, row) in field.iter().enumerate() {
        for (col_index, element) in row.iter().enumerate() {
            if *element {
                max_row = std::cmp::max(max_row, row_index);
                max_col = std::cmp::max(max_col, col_index);
                min_row = std::cmp::min(min_row, row_index);
                min_col = std::cmp::min(min_col, col_index);
                count_elves += 1;
            } 
        }
    }

    (max_row - min_row + 1) * (max_col - min_col + 1) - count_elves
}

fn print_field(field: &VecDeque<VecDeque<bool>>) {

    for row in field {
        let mut s = String::new();
        for element in row {
            if *element {
                s.push('#');
            } else {
                s.push('.');
            }
        }

        println!("{}", s);
    }
}

fn pad_field(field: &mut VecDeque<VecDeque<bool>>) {

    if field[0].iter().any(|e| *e) {
        add_top(field);
    }

    if field.range(1..field.len() - 1).any(|row| row[0]) {
        add_left(field);
    }

    if field.range(1..field.len() - 1).any(|row| row[row.len() - 1]) {
        add_right(field);
    }

    if field[field.len() - 1].iter().any(|e| *e) {
        add_bottom(field);
    }
}

fn add_left(field: &mut VecDeque<VecDeque<bool>>) {

    for row in field {
        row.push_front(false);
    }
}

fn add_right(field: &mut VecDeque<VecDeque<bool>>) {

    for row in field {
        row.push_back(false);
    }
}

fn add_bottom(field: &mut VecDeque<VecDeque<bool>>) {

    let mut extra_row = VecDeque::new();

    for _ in 0..field[0].len(){
        extra_row.push_back(false);
    }

    field.push_back(extra_row);
}

fn add_top(field: &mut VecDeque<VecDeque<bool>>) {

    let mut extra_row = VecDeque::new();

    for _ in 0..field[0].len() {
        extra_row.push_back(false);
    }

    field.push_front(extra_row);
}


fn main() {

    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();

    let mut field: VecDeque<VecDeque<bool>> = VecDeque::new();

    for line in lines {
        let mut row= VecDeque::new();

        for c in line.chars() {
            row.push_back(c=='#');
        }

        field.push_back(row);
    }

    println!("number of empty squares {}", spread(field));
 
}