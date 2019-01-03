fn display_board(board: &[[char; 3]; 3]) {
    for row in board.iter() {
        println!("{:?}", row);
    }
}

fn get_input() -> (i32, i32) {

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");

    let mut inputs = input.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Parse error"))
        .collect::<Vec<i32>>()
        .retain(|&i| (-1 < i) & (i < 3));

    if inputs.len() != 2 {
        return (-1, -1);
    } else {
        return (inputs[1], inputs[0]);
    }

}

fn take_turn(player:char, board: &mut [[char; 3]; 3]) -> (i32, i32) {
    display_board(&board);

    let mut valid = false;
    let mut input:(i32, i32) = (-1,-1);

    while !valid {
        println!("Player {}, Enter coordinates x y:", player);
        input = get_input();
        let loc = board[input.0 as usize][input.1 as usize];
        if loc == 'N' {
            board[input.0 as usize][input.1 as usize] = player;
            valid = true;
        } else {
            println!("Invalid: {} in that position", loc)
        }
    };

    return input
}

fn check_board(board: &[[char; 3]; 3], loc: (i32, i32)) -> bool {
    /* check for win conditions */
    let (x, y) = (loc.0 as usize, loc.1 as usize);

    if (board[x][0] == board[x][1]) & (board[x][0] == board[x][2]) {
        return true;
    }

    if (board[0][y] == board[1][y]) & (board[0][y] == board[2][y]) {
        return true;
    }

    if (x == y) & (board[0][0] == board[1][1]) & (board[0][0] == board[2][2]) {
        return true;
    }

    if (x + y == 2) & (board[0][2] == board[1][1]) & (board[0][2] == board[2][0]) {
        return true;
    }

    return false;
}

fn main() {

    let mut board:[[char; 3]; 3] = [['N', 'N', 'N'],['N', 'N', 'N'],['N', 'N', 'N']];

    loop {
        let p1 = take_turn('X', &mut board);

        if check_board(&board, p1) {
            println!("p1 win!");
            break;
        }

        let p2 = take_turn('O', &mut board);

        if check_board(&board, p2) {
            println!("p2 win!");
            break;
        }
    }
}
