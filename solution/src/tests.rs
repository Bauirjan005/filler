use crate::board::{Board, Player};
use crate::parser::{
    format_coordinates, parse_board_header, parse_board_row, parse_piece_header, parse_player,
    parse_turn,
};
use crate::piece::Piece;
use crate::strategy::choose_best_move;
use std::io::Cursor;

/// 1. Verify Input Parsing
/// Ensures the Anfield grid and incoming pieces are correctly converted into internal data structures.
#[test]
fn test_input_parsing() {
    let input = b"$$$ exec p1 : [robots/bender]\n\
Anfield 20 15:\n\
    01234567890123456789\n\
000 ....................\n\
001 ....................\n\
002 .........@..........\n\
003 ....................\n\
004 ....................\n\
005 ....................\n\
006 ....................\n\
007 ....................\n\
008 ....................\n\
009 ....................\n\
010 ....................\n\
011 ....................\n\
012 .........$..........\n\
013 ....................\n\
014 ....................\n\
Piece 4 1:\n\
.OO.\n";

    let mut cursor = Cursor::new(&input[..]);
    let mut player = None;

    let result = parse_turn(&mut cursor, &mut player).expect("Parsing should succeed");
    assert!(result.is_some(), "Turn data should be parsed");

    let (board, piece) = result.unwrap();

    // Verify player parsing
    assert_eq!(player, Some(Player::Player1), "Player should be parsed as Player 1");

    // Verify board dimensions
    assert_eq!(board.width, 20, "Board width must be 20");
    assert_eq!(board.height, 15, "Board height must be 15");

    // Verify cell contents
    assert_eq!(board.grid[2][9], '@', "Cell at (9, 2) should be '@'");
    assert_eq!(board.grid[12][9], '$', "Cell at (9, 12) should be '$'");
    assert_eq!(board.grid[0][0], '.', "Cell at (0, 0) should be empty '.'");

    // Verify piece parsing
    assert_eq!(piece.width, 4, "Piece width must be 4");
    assert_eq!(piece.height, 1, "Piece height must be 1");
    assert_eq!(piece.shape.len(), 2, "Piece should have 2 solid shape cells");
    assert_eq!(
        piece.shape,
        vec![(1, 0), (2, 0)],
        "Solid cells should be at offsets (1, 0) and (2, 0)"
    );
}

/// Helper parser tests
#[test]
fn test_individual_parsers() {
    // Player parser
    assert_eq!(parse_player("$$$ exec p1 : [robots/bender]"), Some(Player::Player1));
    assert_eq!(parse_player("$$$ exec p2 : [robots/terminator]"), Some(Player::Player2));
    assert_eq!(parse_player("Random line"), None);

    // Board header parser
    assert_eq!(parse_board_header("Anfield 20 15:"), Some((20, 15)));
    assert_eq!(parse_board_header("Plateau 30 14:"), Some((30, 14)));
    assert_eq!(parse_board_header("Invalid"), None);

    // Piece header parser
    assert_eq!(parse_piece_header("Piece 4 1:"), Some((4, 1)));
    assert_eq!(parse_piece_header("Piece 5 4:"), Some((5, 4)));
    assert_eq!(parse_piece_header("Anfield 20 15:"), None);

    // Board row parser
    let row = parse_board_row("002 .........@..........", 20).unwrap();
    assert_eq!(row.len(), 20);
    assert_eq!(row[9], '@');
}

/// 2. Verify Valid Placement Logic
/// Tests that a potential coordinate correctly identifies if it has exactly one overlapping
/// cell with existing territory and zero overlaps with the opponent.
#[test]
fn test_valid_placement_logic() {
    let mut grid = vec![vec!['.'; 20]; 15];
    grid[2][9] = '@'; // Player 1 territory
    grid[12][9] = '$'; // Player 2 territory
    let board = Board::new(20, 15, grid);

    // Piece: .OO. (shape at offsets (1, 0) and (2, 0))
    let piece = Piece::new(4, 1, vec![".OO.".to_string()]);

    // Test Valid Move: (7, 2)
    // Shape cell (1, 0) is at (8, 2) -> empty '.'
    // Shape cell (2, 0) is at (9, 2) -> '@' (Player 1 territory)
    // Exactly 1 overlap with own territory, 0 with opponent.
    assert!(
        piece.is_valid_placement(&board, &Player::Player1, 7, 2),
        "Placement at (7, 2) should be valid for Player 1"
    );

    // Test Invalid: 0 overlaps with own territory
    assert!(
        !piece.is_valid_placement(&board, &Player::Player1, 0, 0),
        "Placement with 0 overlaps should be invalid"
    );

    // Test Invalid: Overlap with opponent territory
    // Placing at (7, 12) causes shape cell (2, 0) to hit (9, 12) which is '$'
    assert!(
        !piece.is_valid_placement(&board, &Player::Player1, 7, 12),
        "Placement overlapping opponent piece must be invalid"
    );

    // Test Invalid: More than 1 overlap with own territory
    let mut double_grid = vec![vec!['.'; 20]; 15];
    double_grid[2][8] = '@';
    double_grid[2][9] = '@';
    let board_double = Board::new(20, 15, double_grid);
    assert!(
        !piece.is_valid_placement(&board_double, &Player::Player1, 7, 2),
        "Placement overlapping 2 cells of own territory must be invalid"
    );
}

/// 3. Test Boundary Conditions
/// Ensures the robot never attempts to place a piece that extends beyond the rows or columns of the Anfield.
#[test]
fn test_boundary_conditions() {
    let mut grid = vec![vec!['.'; 20]; 15];
    grid[0][0] = '@';
    grid[14][19] = '@';
    let board = Board::new(20, 15, grid);

    let piece = Piece::new(4, 1, vec![".OO.".to_string()]);

    // Shape cells are at dx = 1 and dx = 2.
    // Right boundary: x = 18 means dx = 2 hits bx = 20, which is >= width (20)
    assert!(
        !piece.is_valid_placement(&board, &Player::Player1, 18, 14),
        "Piece extending beyond right edge must be rejected"
    );

    // Left boundary: x = -2 means dx = 1 hits bx = -1, which is < 0
    assert!(
        !piece.is_valid_placement(&board, &Player::Player1, -2, 0),
        "Piece extending beyond left edge must be rejected"
    );

    // Bottom boundary: y = 15 is >= height (15)
    assert!(
        !piece.is_valid_placement(&board, &Player::Player1, 0, 15),
        "Piece extending below bottom edge must be rejected"
    );

    // Top boundary: y = -1 is < 0
    assert!(
        !piece.is_valid_placement(&board, &Player::Player1, 0, -1),
        "Piece extending above top edge must be rejected"
    );

    // Verify get_valid_placements only returns coordinates that are within boundary
    let placements = piece.get_valid_placements(&board, &Player::Player1);
    for &(x, y) in &placements {
        for &(dx, dy) in &piece.shape {
            let bx = x + dx as isize;
            let by = y + dy as isize;
            assert!(
                bx >= 0 && bx < 20 && by >= 0 && by < 15,
                "Valid placement ({}, {}) produces out of bounds cell ({}, {})",
                x,
                y,
                bx,
                by
            );
        }
    }
}

/// 4. Verify Coordinate Output
/// Ensures the final calculation is formatted exactly as X Y\n as required by the game_engine.
#[test]
fn test_coordinate_output() {
    assert_eq!(format_coordinates(7, 2), "7 2\n");
    assert_eq!(format_coordinates(0, 0), "0 0\n");
    assert_eq!(format_coordinates(-3, 10), "-3 10\n");
    assert_eq!(format_coordinates(19, 14), "19 14\n");
}

/// Test strategy move selection on the sample board
#[test]
fn test_strategy_selects_valid_move() {
    let mut grid = vec![vec!['.'; 20]; 15];
    grid[2][9] = '@';
    grid[12][9] = '$';
    let board = Board::new(20, 15, grid);

    let piece = Piece::new(4, 1, vec![".OO.".to_string()]);
    let (x, y) = choose_best_move(&board, &piece, &Player::Player1);

    assert!(
        piece.is_valid_placement(&board, &Player::Player1, x, y),
        "Chosen move ({}, {}) must be a valid placement",
        x,
        y
    );
}

/// Test fallback when no moves are possible
#[test]
fn test_no_moves_returns_zero_zero() {
    // Isolated board with no territory for player 1
    let grid = vec![vec!['.'; 10]; 10];
    let board = Board::new(10, 10, grid);
    let piece = Piece::new(2, 2, vec!["##".to_string(), "##".to_string()]);

    let mv = choose_best_move(&board, &piece, &Player::Player1);
    assert_eq!(
        mv,
        (0, 0),
        "When no moves are possible, (0, 0) should be returned"
    );
}

/// Test Player 2 perspective
#[test]
fn test_player2_handling() {
    let p2 = Player::Player2;
    assert!(p2.is_my_cell('$'));
    assert!(p2.is_my_cell('s'));
    assert!(p2.is_opponent_cell('@'));
    assert!(p2.is_opponent_cell('a'));

    let mut grid = vec![vec!['.'; 20]; 15];
    grid[2][9] = '@';
    grid[12][9] = '$';
    let board = Board::new(20, 15, grid);

    let piece = Piece::new(4, 1, vec![".OO.".to_string()]);
    // For Player 2, placing at (7, 12) overlaps (9, 12) which is '$'
    assert!(piece.is_valid_placement(&board, &Player::Player2, 7, 12));
    assert!(!piece.is_valid_placement(&board, &Player::Player2, 7, 2));
}
