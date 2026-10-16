use std::collections::VecDeque;

/// Represents Player 1 (@, a) or Player 2 ($, s).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Player1, // '@' (territory), 'a' (last placed piece)
    Player2, // '$' (territory), 's' (last placed piece)
}

#[allow(dead_code)]
impl Player {
    /// Returns true if the character belongs to this player's territory.
    pub fn is_my_cell(&self, c: char) -> bool {
        match self {
            Player::Player1 => c == '@' || c == 'a',
            Player::Player2 => c == '$' || c == 's',
        }
    }

    /// Returns true if the character belongs to the opponent's territory.
    pub fn is_opponent_cell(&self, c: char) -> bool {
        match self {
            Player::Player1 => c == '$' || c == 's',
            Player::Player2 => c == '@' || c == 'a',
        }
    }

    /// Returns true if the character is an empty cell.
    pub fn is_empty(c: char) -> bool {
        c == '.'
    }

    /// Returns the character symbols used for this player (main, last_placed).
    pub fn symbols(&self) -> (char, char) {
        match self {
            Player::Player1 => ('@', 'a'),
            Player::Player2 => ('$', 's'),
        }
    }

    /// Returns the opponent Player variant.
    pub fn opponent(&self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

/// The Anfield grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
}

#[allow(dead_code)]
impl Board {
    /// Creates a new Board with the given dimensions and character grid.
    pub fn new(width: usize, height: usize, grid: Vec<Vec<char>>) -> Self {
        Self { width, height, grid }
    }

    /// Safely gets the character at (x, y) if within bounds.
    pub fn get(&self, x: isize, y: isize) -> Option<char> {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            Some(self.grid[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Returns a list of all coordinates (x, y) occupied by this player.
    pub fn my_positions(&self, player: &Player) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if player.is_my_cell(self.grid[y][x]) {
                    positions.push((x, y));
                }
            }
        }
        positions
    }

    /// Returns a list of all coordinates (x, y) occupied by the opponent.
    pub fn opponent_positions(&self, player: &Player) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if player.is_opponent_cell(self.grid[y][x]) {
                    positions.push((x, y));
                }
            }
        }
        positions
    }

    /// Computes a BFS heatmap of shortest Manhattan distances to the nearest opponent cell.
    /// Opponent cells have distance 0. Cells adjacent to opponent have distance 1, etc.
    pub fn compute_heatmap(&self, player: &Player) -> Vec<Vec<i32>> {
        let mut dist = vec![vec![i32::MAX; self.width]; self.height];
        let mut queue = VecDeque::new();

        let opp_positions = self.opponent_positions(player);
        for &(x, y) in &opp_positions {
            dist[y][x] = 0;
            queue.push_back((x, y));
        }

        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((cx, cy)) = queue.pop_front() {
            let current_dist = dist[cy][cx];
            for &(dx, dy) in &directions {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if nx >= 0 && (nx as usize) < self.width && ny >= 0 && (ny as usize) < self.height {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    if dist[uy][ux] > current_dist + 1 {
                        dist[uy][ux] = current_dist + 1;
                        queue.push_back((ux, uy));
                    }
                }
            }
        }

        dist
    }

    /// Computes territory control (Voronoi partition count):
    /// Returns (my_controlled_cells, opponent_controlled_cells) among empty cells.
    pub fn compute_territory_control(&self, player: &Player) -> (usize, usize) {
        let mut my_dist = vec![vec![i32::MAX; self.width]; self.height];
        let mut opp_dist = vec![vec![i32::MAX; self.width]; self.height];
        let mut my_queue = VecDeque::new();
        let mut opp_queue = VecDeque::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if player.is_my_cell(self.grid[y][x]) {
                    my_dist[y][x] = 0;
                    my_queue.push_back((x, y));
                } else if player.is_opponent_cell(self.grid[y][x]) {
                    opp_dist[y][x] = 0;
                    opp_queue.push_back((x, y));
                }
            }
        }

        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        // BFS for my distances
        while let Some((cx, cy)) = my_queue.pop_front() {
            let d = my_dist[cy][cx];
            for &(dx, dy) in &directions {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if nx >= 0 && (nx as usize) < self.width && ny >= 0 && (ny as usize) < self.height {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    if my_dist[uy][ux] > d + 1 {
                        my_dist[uy][ux] = d + 1;
                        my_queue.push_back((ux, uy));
                    }
                }
            }
        }

        // BFS for opponent distances
        while let Some((cx, cy)) = opp_queue.pop_front() {
            let d = opp_dist[cy][cx];
            for &(dx, dy) in &directions {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if nx >= 0 && (nx as usize) < self.width && ny >= 0 && (ny as usize) < self.height {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    if opp_dist[uy][ux] > d + 1 {
                        opp_dist[uy][ux] = d + 1;
                        opp_queue.push_back((ux, uy));
                    }
                }
            }
        }

        let mut my_count = 0;
        let mut opp_count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if Player::is_empty(self.grid[y][x]) {
                    if my_dist[y][x] < opp_dist[y][x] {
                        my_count += 1;
                    } else if opp_dist[y][x] < my_dist[y][x] {
                        opp_count += 1;
                    }
                }
            }
        }

        (my_count, opp_count)
    }
}
