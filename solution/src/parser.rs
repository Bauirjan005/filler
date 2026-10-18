use crate::board::{Board, Player};
use crate::piece::Piece;
use std::io::BufRead;

pub fn parse_player(line: &str) -> Option<Player> {
    if !line.contains("$$$") {
        return None;
    }
    if line.contains("p1") {
        Some(Player::Player1)
    } else if line.contains("p2") {
        Some(Player::Player2)
    } else {
        None
    }
}

pub fn parse_board_header(line: &str) -> Option<(usize, usize)> {
    let mut it = line.split_whitespace();
    let name = it.next()?.to_lowercase();
    (name.starts_with("anfield") || name.starts_with("plateau"))
        .then(|| Some((it.next()?.parse().ok()?, it.next()?.trim_end_matches(':').parse().ok()?)))
        .flatten()
}

pub fn parse_piece_header(line: &str) -> Option<(usize, usize)> {
    let mut it = line.split_whitespace();
    it.next()?.eq_ignore_ascii_case("piece")
        .then(|| Some((it.next()?.parse().ok()?, it.next()?.trim_end_matches(':').parse().ok()?)))
        .flatten()
}

pub fn parse_board_row(line: &str, expected_width: usize) -> Option<Vec<char>> {
    let row = line.trim_end().split_whitespace().last()?;
    (row.len() >= expected_width).then(|| row.chars().take(expected_width).collect())
}

pub fn format_coordinates(x: isize, y: isize) -> String {
    format!("{x} {y}\n")
}

pub fn parse_turn<R: BufRead>(
    reader: &mut R,
    player: &mut Option<Player>,
) -> Result<Option<(Board, Piece)>, String> {
    let mut next_line = || -> Option<String> {
        let mut s = String::new();
        (reader.read_line(&mut s).ok()? > 0).then_some(s)
    };

    let (bw, bh) = loop {
        let line = match next_line() {
            Some(l) => l,
            None => return Ok(None),
        };
        if player.is_none() {
            *player = parse_player(&line).or(*player);
        }
        if let Some(dim) = parse_board_header(&line) {
            break dim;
        }
    };

    next_line(); // skip column index header
    let grid = (0..bh)
        .map(|_| parse_board_row(&next_line().unwrap_or_default(), bw).unwrap_or_default())
        .collect();

    let (pw, ph) = loop {
        let line = next_line().ok_or("Unexpected EOF before Piece")?;
        if let Some(dim) = parse_piece_header(&line) {
            break dim;
        }
    };

    let raw = (0..ph)
        .map(|_| next_line().unwrap_or_default().trim_end().chars().take(pw).collect())
        .collect();

    Ok(Some((Board::new(bw, bh, grid), Piece::new(pw, ph, raw))))
}
