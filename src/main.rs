

fn print_board(board: [[usize; 9]; 9]) {
    println!("+-------+-------+-------+");
    let mut i = 0;
    for row in board {
        print!("|");
        let mut j = 0;
        for value in row {
            if value == 0 {
                print!("  ")
            } else {
                print!(" {}", value);
            }
            if j == 2 || j == 5 || j == 8 {
                print!(" |")
            }
            j = j + 1;
        }
        println!("");
        if i == 2 || i == 5 || i == 8 {
            println!("+-------+-------+-------+")
        }
        i = i + 1;
    }
}

fn main() {
    let board: [[usize; 9]; 9] = [[0; 9]; 9];
    print_board(board);
}
