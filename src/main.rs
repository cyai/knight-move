use std::process::Command;
use std::time::Duration;
use std::{thread, vec};

fn print_board(size_x: i32, _size_y: i32, board: &Vec<Vec<i32>>) {
    println!("Knight's Tour Problem");
    println!("Chess Board:");
    print!(" ");
    for _ in 0..size_x {
        print!("___ ");
    }
    println!();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!("| {} ", if (board[i][j]) == -1 { " " } else { "X" });
        }
        println!("|");
        print!(" ");
        for _ in 0..size_x {
            print!("--- ");
        }
        println!();
    }

    println!();
    println!();
    println!();

    println!("Knight's Tour Path:");
    print!(" ");
    for j in 0..size_x {
        print!(
            "{}",
            if board[0][j as usize] < 10 {
                "___ "
            } else {
                "____ "
            }
        );
    }
    println!();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!(
                "| {} ",
                if (board[i][j]) == -1 {
                    " ".to_string()
                } else {
                    board[i][j].to_string()
                }
            );
        }
        println!("|");
        print!(" ");
        for j in 0..size_x {
            print!(
                "{}",
                if board[i][j as usize] < 10 {
                    "--- "
                } else {
                    "---- "
                }
            );
        }
        println!();
    }
}

fn is_valid_move(_size_x: i32, _size_y: i32, board: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
    return x >= 0
        && x < board.len() as i32
        && y >= 0
        && y < board[0].len() as i32
        && board[x as usize][y as usize] == -1;
}

fn solve_knight_tour(
    mut _solution_board: &mut Vec<Vec<i32>>,
    mut _x_move: [i32; 8],
    mut _y_move: [i32; 8],
    mut _move_count: i32,
    mut _x: i32,
    mut _y: i32,
    size_x: usize,
    size_y: usize,
) -> bool {
    let mut next_x: i32;
    let mut next_y: i32;

    if _move_count == size_x as i32 * size_y as i32 {
        return true;
    }

    for i in 0..8 {
        next_x = _x + _x_move[i];
        next_y = _y + _y_move[i];

        if next_x >= 0 && next_x < size_x as i32 && next_y >= 0 && next_y < size_y as i32 {
            if is_valid_move(
                size_x as i32,
                size_y as i32,
                &_solution_board,
                next_x,
                next_y,
            ) {
                _solution_board[next_x as usize][next_y as usize] = _move_count;

                let _ = Command::new("clear").status();
                print_board(size_x as i32, size_y as i32, &_solution_board);
                thread::sleep(Duration::from_secs_f32(0.2));
                if solve_knight_tour(
                    _solution_board,
                    _x_move,
                    _y_move,
                    _move_count + 1,
                    next_x,
                    next_y,
                    size_x,
                    size_y,
                ) {
                    return true;
                } else {
                    _solution_board[next_x as usize][next_y as usize] = -1;
                }
            }
        }
    }

    return false;
}

fn main() {
    const SIZE_X: usize = 5;
    const SIZE_Y: usize = 6;
    let mut solution_board = vec![vec![-1; SIZE_X]; SIZE_Y];
    let possible_x_moves: [i32; 8] = [2, 1, -1, -2, -2, -1, 1, 2];
    let possible_y_moves: [i32; 8] = [1, 2, 2, 1, -1, -2, -2, -1];

    solution_board[0][0] = 0;

    if solve_knight_tour(
        &mut solution_board,
        possible_x_moves,
        possible_y_moves,
        1,
        0,
        0,
        SIZE_X,
        SIZE_Y,
    ) == false
    {
        let _ = Command::new("clear").status();
        println!("Solution does not exist for the given board dimensions.");
    } else {
        let _ = Command::new("clear").status();
        print_board(SIZE_X as i32, SIZE_Y as i32, &solution_board);
    }
}
