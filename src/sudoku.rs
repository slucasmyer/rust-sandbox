use rand::Rng;
use std::vec::Vec;

pub fn make_board(cell_size: usize) -> Vec<Vec<usize>> {
    let board_size: usize = cell_size*cell_size;
    let mut board: Vec<Vec<usize>> = vec![vec![0; board_size]; board_size];

    // Fill the board with random numbers
    for i in 0..board_size {
        for j in 0..board_size {
            board[i][j] = rand::thread_rng().gen_range(0..board_size);
        }
    }

    board
}

pub fn print_board(board: &Vec<Vec<usize>>) {
    for row in board {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}


pub fn solve_board(board: &mut Vec<Vec<usize>>, cell_size: usize) -> bool {
    let mut row = 0;
    let mut col = 0;
    let mut found_empty = false;
    let board_size: usize = cell_size*cell_size;

    // Find the first empty cell
    for i in 0..board_size {
        for j in 0..board_size {
            if board[i][j] == 0 {
                row = i;
                col = j;
                found_empty = true;
                break;
            }
        }
        if found_empty {
            break;
        }
    }

    // If there are no empty cells, the board is solved
    if !found_empty {
        return true;
    }

    // Try filling the empty cell with a number from 1 to 9
    for num in 1..=board_size {
        if is_valid(board, row, col, num, cell_size) {
            board[row][col] = num;

            if solve_board(board, cell_size) {
                return true;
            }

            // Reset the cell if the current number doesn't lead to a solution
            board[row][col] = 0;
        }
    }

    // No solution was found
    return false;
}

fn is_valid(board: &Vec<Vec<usize>>, row: usize, col: usize, num: usize, cell_size: usize) -> bool {
    // Check if the number is already in the row
    for i in 0..board.len() {
        if board[row][i] == num {
            return false;
        }
    }

    // Check if the number is already in the column
    for i in 0..board.len() {
        if board[i][col] == num {
            return false;
        }
    }

    // Check if the number is already in the 3x3 subgrid
    let start_row = (row / cell_size) * cell_size;
    let start_col = (col / cell_size) * cell_size;
    for i in start_row..start_row + cell_size {
        for j in start_col..start_col + cell_size {
            if board[i][j] == num {
                return false;
            }
        }
    }

    // The number is not in the row, column, or subgrid, so it is valid
    return true;
}
