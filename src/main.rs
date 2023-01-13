use ::std::*;
//use::std::io;

mod sudoku;

fn input (message: &str, expect_msg: &str) -> String {
    println!("{}", message);
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect(expect_msg);
    ret
}

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

fn main() {

    read!(x as u32);
    read!(y as f64);
    read!(z as char);
    println!("{} {} {}", x, y, z);
    println!("{} {} {}", x, y, z);
    println!("{} {} {}", x, y, z);
    println!("{} {} {}", x, y, z);
    let input_string: String = input("enter a cell size", "Failed to read line");
    let input_int: usize = input_string.trim().parse::<usize>().expect("Failed to parse input");
    println!("input_string: {}, input_int: {}", input_string, input_int);
    //const CELL_SIZE: usize = 3;
    let mut board = sudoku::make_board(input_int);
    sudoku::print_board(&board);
    println!("Solving board...");
    if sudoku::solve_board(&mut board, input_int) {
        // Print the solved board
        for row in &board {
            for val in row {
                print!("{} ", val);
            }
            println!();
        }
    } else {
        println!("No solution found");
    }
}

