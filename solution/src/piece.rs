use crate::board::{Board, Player};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub shape: Vec<(usize, usize)>,
    pub raw: Vec<String>,
}

impl Piece {
    pub fn new(width: usize, height: usize, raw: Vec<String>) -> Self {
        let shape = raw
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| (c != '.').then_some((x, y)))
            })
            .collect();
        Self { width, height, shape, raw }
    }

    pub fn is_valid_placement(&self, b: &Board, p: &Player, x: isize, y: isize) -> bool {
        let cells: Vec<_> = self
            .shape
            .iter()
            .map(|&(dx, dy)| (x + dx as isize, y + dy as isize))
            .collect();

        cells.iter().all(|&(bx, by)| {
            bx >= 0
                && bx < b.width as isize
                && by >= 0
                && by < b.height as isize
                && !p.is_opponent_cell(b.grid[by as usize][bx as usize])
        }) && cells
            .iter()
            .filter(|&&(bx, by)| p.is_my_cell(b.grid[by as usize][bx as usize]))
            .count() == 1
    }

    pub fn get_valid_placements(&self, board: &Board, player: &Player) -> Vec<(isize, isize)> {
        (-(self.height as isize)..=board.height as isize)
            .flat_map(|y| (-(self.width as isize)..=board.width as isize).map(move |x| (x, y)))
            .filter(|&(x, y)| self.is_valid_placement(board, player, x, y))
            .collect()
    }

    pub fn new_cells_at(&self, board: &Board, player: &Player, x: isize, y: isize) -> Vec<(usize, usize)> {
        self.shape
            .iter()
            .map(|&(dx, dy)| ((x + dx as isize) as usize, (y + dy as isize) as usize))
            .filter(|&(bx, by)| !player.is_my_cell(board.grid[by][bx]))
            .collect()
    }
}
