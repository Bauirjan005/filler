use crate::board::{Board, Player};
use crate::piece::Piece;
use std::io::BufRead;

/// Parses player from the initial line sent by the game engine, e.g.:
/// `$$$ exec p1 : [robots/bender]`
pub fn parse_player(line: &str) -> Option<Player> {
    if line.contains("$$$") {
        if line.contains("p1") {
            return Some(Player::Player1);
        } else if line.contains("p2") {
            return Some(Player::Player2);
        }
    }
    None
}

/// Parses the board dimensions from `Anfield <width> <height>:` or `Plateau <width> <height>:`.
/// In 01-edu filler: first number is width (columns), second is height (rows).
pub fn parse_board_header(line: &str) -> Option<(usize, usize)> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() >= 3 {
        let name = tokens[0].to_lowercase();
        if name.starts_with("anfield") || name.starts_with("plateau") {
            let width = tokens[1].trim_end_matches(':').parse::<usize>().ok()?;
            let height = tokens[2].trim_end_matches(':').parse::<usize>().ok()?;
            return Some((width, height));
        }
    }
    None
}

/// Parses the piece dimensions from `Piece <width> <height>:`.
/// First number is width (columns), second is height (rows).
pub fn parse_piece_header(line: &str) -> Option<(usize, usize)> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() >= 3 && tokens[0].eq_ignore_ascii_case("piece") {
        let width = tokens[1].trim_end_matches(':').parse::<usize>().ok()?;
        let height = tokens[2].trim_end_matches(':').parse::<usize>().ok()?;
        return Some((width, height));
    }
    None
}

/// Parses a single grid row string from line formatted like `000 ....................`.
pub fn parse_board_row(line: &str, expected_width: usize) -> Option<Vec<char>> {
    let trimmed = line.trim_end_matches(['\r', '\n']);
    // Strip leading spaces and row number digits: e.g. "000 ...."
    let grid_str = trimmed
        .trim_start()
        .trim_start_matches(|c: char| c.is_ascii_digit())
        .trim_start();

    if grid_str.len() >= expected_width {
        let chars: Vec<char> = grid_str.chars().take(expected_width).collect();
        Some(chars)
    } else {
        None
    }
}

/// Formats the calculated coordinates as `X Y\n` as required by the game engine.
pub fn format_coordinates(x: isize, y: isize) -> String {
    format!("{} {}\n", x, y)
}

/// Reads the next turn data (Board and Piece) from stdin.
/// Updates the `player` if the player assignment line was encountered.
/// Returns Ok(Some((Board, Piece))) on a successful turn,
/// Ok(None) on EOF (end of game), or Err on parse error.
pub fn parse_turn<R: BufRead>(
    reader: &mut R,
    player: &mut Option<Player>,
) -> Result<Option<(Board, Piece)>, String> {
    let mut line = String::new();

    // 1. Read until we find the Anfield board header or EOF
    let (board_width, board_height) = loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            return Ok(None); // EOF
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Check if this line is the player announcement
        if player.is_none() {
            if let Some(p) = parse_player(trimmed) {
                *player = Some(p);
                continue;
            }
        }

        // Check if this line is the board header
        if let Some((w, h)) = parse_board_header(trimmed) {
            break (w, h);
        }
    };

    // 2. Read the column index line (e.g. "    0123456789...")
    line.clear();
    let bytes_read = reader.read_line(&mut line).map_err(|e| e.to_string())?;
    if bytes_read == 0 {
        return Err("Unexpected EOF reading column index line".to_string());
    }

    // 3. Read the board grid rows
    let mut grid = Vec::with_capacity(board_height);
    for row_idx in 0..board_height {
        line.clear();
        let bytes_read = reader.read_line(&mut line).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            return Err(format!("Unexpected EOF reading board row {}", row_idx));
        }

        let row_chars = parse_board_row(&line, board_width)
            .ok_or_else(|| format!("Invalid board row at index {}: {}", row_idx, line.trim()))?;
        grid.push(row_chars);
    }
    let board = Board::new(board_width, board_height, grid);

    // 4. Read until Piece header is found
    let (piece_width, piece_height) = loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            return Err("Unexpected EOF before Piece header".to_string());
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some((pw, ph)) = parse_piece_header(trimmed) {
            break (pw, ph);
        }
    };

    // 5. Read the piece rows
    let mut piece_rows = Vec::with_capacity(piece_height);
    for p_idx in 0..piece_height {
        line.clear();
        let bytes_read = reader.read_line(&mut line).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            return Err(format!("Unexpected EOF reading piece row {}", p_idx));
        }
        let trimmed_row = line.trim_end_matches(['\r', '\n']);
        let row_str: String = trimmed_row.chars().take(piece_width).collect();
        piece_rows.push(row_str);
    }
    let piece = Piece::new(piece_width, piece_height, piece_rows);

    Ok(Some((board, piece)))
}
