pub fn fen_parse(fen: String) -> [[String; 8]; 8] {
    let fen_content = &fen[6..];
    println!("{}", fen_content);
    //let mut fen_content = fen_content.split('/');

    //let mut count: u8 = 0;
    let chartable = ['r', 'n', 'b', 'q', 'k', 'p'];

    let mut board: [[String; 8]; 8] = Default::default();
    for i in 0..8 {
        for j in 0..8 {
            board[i][j] = "None".to_owned();
        }
    }

    let mut i: usize = 0;
    let mut j: usize = 0;
    let fen_content = fen_content.split_whitespace().nth(0).unwrap();
    println!("{}", fen_content);
    for character in fen_content.to_string().chars() {
        if character == '/' {
            j = 0;
            i += 1;
        } else {
            if character.is_digit(10) {
                let character = character.to_digit(10).unwrap() as usize;
                j += character;
                //board[i][j] = "None".to_owned()
            } else {
                let color = match character.is_lowercase() {
                    true => "d",
                    false => "l",
                    _ => "d",
                };
                let piece = format!("{}{}", character.to_lowercase(), color);
                board[i][j] = piece;
                j += 1;
            }
        }
    }
    board
}

pub fn to_fen(board: &[[String; 8]; 8], turn: String) -> String {
    let mut fen = String::new();
    let mut non_counter = 0;
    for i in 0..8 {
        for j in 0..8 {
            if &board[i][j] == "None" {
                non_counter += 1;
            } else {
                if non_counter > 0 {
                    fen = format!("{}{}", fen, non_counter)
                }
                non_counter = 0;
                let piece = &board[i][j];
                let color = &piece[1..2];
                let kind = &piece[0..1];
                let kind = match color {
                    "l" => kind.to_uppercase(),
                    "d" => kind.to_lowercase(),
                    _ => kind.to_string(),
                };
                fen = format!("{}{}", fen, kind);
            }
        }
        if non_counter > 0 {
            fen = format!("{}{}", fen, non_counter);
            non_counter = 0;
        }
        if i < 7 {
            fen = format!("{}/", fen);
        }
    }
    fen = format!("board:{} {} - - 0 0;", fen, turn);
    fen
}
