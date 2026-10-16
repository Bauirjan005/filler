use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn is_my_cell(&self, c: char) -> bool {
        matches!((self, c), (Self::Player1, '@' | 'a') | (Self::Player2, '$' | 's'))
    }

    pub fn is_opponent_cell(&self, c: char) -> bool {
        matches!((self, c), (Self::Player1, '$' | 's') | (Self::Player2, '@' | 'a'))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
}

impl Board {
    pub fn new(width: usize, height: usize, grid: Vec<Vec<char>>) -> Self {
        Self { width, height, grid }
    }

    pub fn opponent_positions(&self, player: &Player) -> Vec<(usize, usize)> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| player.is_opponent_cell(self.grid[y][x]))
            .collect()
    }

    pub fn compute_heatmap(&self, player: &Player) -> Vec<Vec<i32>> {
        let mut dist = vec![vec![i32::MAX; self.width]; self.height];
        let mut q = VecDeque::new();

        for &(x, y) in &self.opponent_positions(player) {
            dist[y][x] = 0;
            q.push_back((x, y));
        }

        while let Some((cx, cy)) = q.pop_front() {
            let d = dist[cy][cx] + 1;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = (cx as isize + dx, cy as isize + dy);
                if nx >= 0 && (nx as usize) < self.width && ny >= 0 && (ny as usize) < self.height {
                    let (ux, uy) = (nx as usize, ny as usize);
                    if dist[uy][ux] > d {
                        dist[uy][ux] = d;
                        q.push_back((ux, uy));
                    }
                }
            }
        }
        dist
    }
}
