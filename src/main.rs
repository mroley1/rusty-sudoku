extern crate rand;

use rand::seq::SliceRandom;
use std::collections::HashSet;


fn print_board(board: [[usize; 9]; 9]) {
    println!("+-------+-------+-------+");
    for row_index in 0..9 {
        print!("|");
        for col_index in 0..9 {
            let value = board[row_index][col_index];
            if value == 0 {
                print!("  ")
            } else {
                print!(" {}", value);
            }
            if col_index == 2 || col_index == 5 || col_index == 8 {
                print!(" |")
            }
        }
        println!("");
        if row_index == 2 || row_index == 5 || row_index == 8 {
            println!("+-------+-------+-------+")
        }
    }
}

fn fill_diagonals(board: &mut [[usize; 9]; 9]) {
    let mut rng = rand::thread_rng();
    for cell in 0..3 {
        let mut possible = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        possible.shuffle(&mut rng);
        for int in 0..9 {
            let row = (cell * 3) + (int / 3);
            let col = (cell * 3) + (int % 3);
            board[row][col] = possible[int]
        }
    }
}

fn get_possible(board: &mut [[usize; 9]; 9], row: usize, col: usize) -> HashSet<usize> {
    let mut set = HashSet::from([1,2,3,4,5,6,7,8,9]);
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for i in 0..9 {
        set.remove(&board[row][i]);
        set.remove(&board[i][col]);
        set.remove(&board[box_row + (i / 3)][box_col + (i % 3)]);
    }
    set
}

fn complete_box(board: &mut [[usize; 9]; 9], box_row: usize, box_col: usize) {
    for int in 0..9 {
        let row = (box_row * 3) + (int / 3);
        let col = (box_col * 3) + (int % 3);
        board[row][col] = match get_possible(board, row, col).iter().next() {
            Some(r) => *r, // pumpkin time 🎃
            None => 0,
        };
    }
}

fn make_x(board: &mut [[usize; 9]; 9]) {
    complete_box(board, 0, 2);
    complete_box(board, 2, 0);
}

fn main() {
    let mut board: [[usize; 9]; 9] = [[0; 9]; 9];
    fill_diagonals(&mut board);
    print_board(board);
    make_x(&mut board);
    print_board(board);
}
