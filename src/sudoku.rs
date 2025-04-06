fn main() {
    let board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9]
    ];
    if check_sudoku_board(board){
        println!("Sudoku is fine :)")
    } else {
        println!("There are some errors!!! :c")
    };
}
fn check_sudoku_board(board: [[u8; 9]; 9]) -> bool {
    //Rows
    for row in board{
        if !is_valid(row){
            return false;
        }
    }

    //Columns
    for column in 0..9{
        let mut line = [0;9];
        for row in 0..9{
            line[row] = board[row][column];
        }
        if !is_valid(line){
            return false;
        }
    }

    //"Small" squares
    for x_square in 0..3{
        for y_square in 0..3{
            let mut square = [0;9];
            let mut i = 0;
            for x in 0..3{
                for y in 0..3{
                    square[i] = board[y_square * 3 + y][x_square * 3 + x];
                    i += 1;
                }
            }
            if !is_valid(square){
                return false;
            }
        }

    }

    true
}

fn is_valid(group: [u8; 9]) -> bool { //Ensures if each numbers appear only once and is in range 1-9
    let mut used = [false;9];
    for i in group{
        if i != 0 {
            if i < 1 || i > 9 {
                return false;
            }
            if used[(i - 1) as usize]{
                return false;
            }
            used[(i - 1) as usize] = true;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_okay(){
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(check_sudoku_board(board));
    }

    #[test]
    fn test_error_in_row(){
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 3, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!check_sudoku_board(board));
    }

    #[test]
    fn test_error_in_column(){
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [5, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!check_sudoku_board(board));
    }

    #[test]
    fn test_error_in_small_square(){
        let board = [
            [5, 3, 6, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!check_sudoku_board(board));
    }
}