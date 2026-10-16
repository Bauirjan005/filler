use crate::board::{Board, Player};
use crate::piece::Piece;

/// Evaluates and selects the best move for the robot.
pub fn choose_best_move(board: &Board, piece: &Piece, player: &Player) -> (isize, isize) {
    let valid_placements = piece.get_valid_placements(board, player);

    if valid_placements.is_empty() {
        // As specified by the subject: if no moves can be made, return (0, 0)
        return (0, 0);
    }

    let heatmap = board.compute_heatmap(player);
    let opp_positions = board.opponent_positions(player);

    // Compute opponent center of mass if available
    let (opp_cx, opp_cy) = if !opp_positions.is_empty() {
        let sum_x: usize = opp_positions.iter().map(|&(x, _)| x).sum();
        let sum_y: usize = opp_positions.iter().map(|&(_, y)| y).sum();
        (
            (sum_x as f64) / (opp_positions.len() as f64),
            (sum_y as f64) / (opp_positions.len() as f64),
        )
    } else {
        (board.width as f64 / 2.0, board.height as f64 / 2.0)
    };

    let mut best_move = valid_placements[0];
    let mut best_score = i64::MAX;

    for &(x, y) in &valid_placements {
        let new_cells = piece.new_cells_at(board, player, x, y);

        if new_cells.is_empty() {
            continue;
        }

        // Distance metrics to opponent
        let mut min_dist = i32::MAX;
        let mut sum_dist: i64 = 0;
        let mut direct_blocks: i64 = 0; // Number of newly placed cells immediately adjacent to opponent

        for &(cx, cy) in &new_cells {
            let d = heatmap[cy][cx];
            if d < min_dist {
                min_dist = d;
            }
            if d != i32::MAX {
                sum_dist += d as i64;
            }
            if d == 1 {
                direct_blocks += 1;
            }
        }

        // Calculate distance from new cells to opponent's centroid
        let mut centroid_dist = 0.0;
        for &(cx, cy) in &new_cells {
            let dx = cx as f64 - opp_cx;
            let dy = cy as f64 - opp_cy;
            centroid_dist += (dx * dx + dy * dy).sqrt();
        }

        // Combined score (lower is better):
        // 1. min_dist has highest priority: push vanguard directly toward opponent.
        // 2. Direct blocks heavily rewarded during contact phase (subtracting bonus).
        // 3. sum_dist keeps the formation compact and pushing forward.
        // 4. centroid_dist guides toward the core of the opponent territory.
        let score = if min_dist <= 1 {
            // Close combat / walling-in phase:
            // Heavily reward direct blocks (cutting off opponent expansion)
            (min_dist as i64) * 10000 - direct_blocks * 5000 + sum_dist * 10 + (centroid_dist as i64)
        } else {
            // Rushing phase:
            // Priority is minimizing distance to opponent as fast as possible
            (min_dist as i64) * 100000 + sum_dist * 50 + (centroid_dist as i64)
        };

        if score < best_score {
            best_score = score;
            best_move = (x, y);
        }
    }

    best_move
}
