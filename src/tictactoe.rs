use std::io;

fn main() {
    let mut board = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    println!("Welcome in TicTacToe!");
    let mut user_input = String::new();
    let mut curr_player : char;
    println!("Choose starting player:");
    let _ = io::stdin().read_line(&mut user_input);
    curr_player = user_input.chars().nth(0).unwrap();
    let mut curr_player_number = String::from("1");

    {
        print_board(board);
        println!("Player {} ({}), your move (enter field number from 1 to 9):", curr_player_number, curr_player);

        let mut user_input = String::new();
        let mut choice : char;
        let _ = io::stdin().read_line(&mut user_input);
        choice = user_input.chars().nth(0).unwrap();
        // let final_choice: i32 = choice.parse

        board[player_move as usize] = curr_player;
        print_board(board);

    }
}
fn print_board(board: [char; 9]) {
    println!("Board:");
    println!("{} | {} | {}", board[0], board[1] ,board[2]);
    println!("---------");
    println!("{} | {} | {}", board[3], board[4], board[5]);
    println!("---------");
    println!("{} | {} | {}", board[6], board[7], board[8]);
}