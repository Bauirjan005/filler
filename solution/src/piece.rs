use crate::board::{Board, Player};

/// Represents a piece given by the game engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    /// Coordinates (dx, dy) of solid shape cells relative to piece origin (0, 0).
    pub shape: Vec<(usize, usize)>,
    pub raw: Vec<String>,
}

impl Piece {
    /// Creates a Piece from raw grid lines.
    /// Any character other than '.' is treated as a solid block.
    pub fn new(width: usize, height: usize, raw: Vec<String>) -> Self {
        let mut shape = Vec::new();
        for (y, row) in raw.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch != '.' {
                    shape.push((x, y));
                }
            }
        }
        Self {
            width,
            height,
            shape,
            raw,
        }
    }

    /// Checks if placing this piece at (x, y) is valid according to the rules:
    /// 1. Every solid shape cell must be within the Anfield boundaries.
    /// 2. Exactly one solid shape cell must overlap the player's existing territory.
    /// 3. Zero solid shape cells may overlap the opponent's territory.
    pub fn is_valid_placement(&self, board: &Board, player: &Player, x: isize, y: isize) -> bool {
        let mut my_overlaps = 0;

        for &(dx, dy) in &self.shape {
            let bx = x + dx as isize;
            let by = y + dy as isize;

            // Boundary condition: shape cell must be within the board
            if bx < 0 || bx >= board.width as isize || by < 0 || by >= board.height as isize {
                return false;
            }

            let cell = board.grid[by as usize][bx as usize];
            if player.is_opponent_cell(cell) {
                // Must not overlap any opponent cell
                return false;
            } else if player.is_my_cell(cell) {
                my_overlaps += 1;
                if my_overlaps > 1 {
                    // Cannot overlap more than one cell of own territory
                    return false;
                }
            }
        }

        my_overlaps == 1
    }

    /// Finds all valid (x, y) coordinates on the board where this piece can be placed.
    pub fn get_valid_placements(&self, board: &Board, player: &Player) -> Vec<(isize, isize)> {
        let mut valid = Vec::new();
        let min_x = -(self.width as isize);
        let max_x = board.width as isize;
        let min_y = -(self.height as isize);
        let max_y = board.height as isize;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.is_valid_placement(board, player, x, y) {
                    valid.push((x, y));
                }
            }
        }
        valid
    }

    /// Returns the coordinates of newly placed cells (cells that were previously empty)
    /// when the piece is placed at (x, y). Assumes placement is valid.
    pub fn new_cells_at(&self, board: &Board, player: &Player, x: isize, y: isize) -> Vec<(usize, usize)> {
        let mut new_cells = Vec::new();
        for &(dx, dy) in &self.shape {
            let bx = x + dx as isize;
            let by = y + dy as isize;
            if bx >= 0 && (bx as usize) < board.width && by >= 0 && (by as usize) < board.height {
                let ux = bx as usize;
                let uy = by as usize;
                if !player.is_my_cell(board.grid[uy][ux]) {
                    new_cells.push((ux, uy));
                }
            }
        }
        new_cells
    }
}
