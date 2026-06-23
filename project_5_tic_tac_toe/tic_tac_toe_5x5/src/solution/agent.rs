use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use tic_tac_toe_stencil::board::Cell;


pub struct SolutionAgent {}

impl Agent for SolutionAgent {    
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let remaining = board.moves().len();
        let max_depth = if remaining <= 9 { remaining as u32 } else { 4 };
        SolutionAgent::minimax(board, player, 0, max_depth)
    }
}

impl SolutionAgent {
    fn score_window(a: &Cell, b: &Cell, c: &Cell) -> i32 {
        let x_count = [a, b, c].iter().filter(|cell| ***cell == Cell::X).count();
        let o_count = [a, b, c].iter().filter(|cell| ***cell == Cell::O).count();
        
        if x_count > 0 && o_count > 0 {
            0
        } else if x_count > 0 {
            match x_count {
                2 => 3,
                1 => 1,
                _ => 0,
            }
        } else if o_count > 0 {
            match o_count {
                2 => -3,
                1 => -1,
                _ => 0,
            }
        } else {
            0
        }
    }
    
    fn heuristic(board: &Board) -> i32 {
    let mut score = 0;
    
    score += board.score() * 10;
    
    let cells = board.get_cells();
    let size = cells.len();

    for i in 0..size {
        for j in 0..size {
            // check horizontal window
            if j + 2 < size {
                let a = &cells[i][j];
                let b = &cells[i][j+1];
                let c = &cells[i][j+2];
                // analyze a, b, c
                score += Self::score_window(a, b, c);
            }
            // check vertical window
            if i + 2 < size {
                let a = &cells[i][j];
                let b = &cells[i+1][j];
                let c = &cells[i+2][j];
                // analyze a, b, c
                score += Self::score_window(a, b, c);
            }
            // check diagonal down-right
            if i + 2 < size && j + 2 < size {
                let a = &cells[i][j];
                let b = &cells[i+1][j+1];
                let c = &cells[i+2][j+2];
                // analyze a, b, c
                score += Self::score_window(a, b, c);
            }
            // check diagonal down-left
            if i + 2 < size && j >= 2 {
                let a = &cells[i][j];
                let b = &cells[i+1][j-1];
                let c = &cells[i+2][j-2];
                // analyze a, b, c
                score += Self::score_window(a, b, c);
            }
        }
    }
    
    score
}

    fn minimax(board: &mut Board, player: Player, depth: u32, max_depth: u32) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0);
        } 
        else if depth == max_depth {
            return (Self::heuristic(board), 0, 0);
        }

        
        let moves = board.moves(); 

        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        }; 

        let mut best_move = moves[0]; 

        for i in moves {
            board.apply_move(i, player); 

            let next_player = match player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            
            let (score, _, _) = SolutionAgent::minimax(board, next_player, depth + 1, max_depth);  

            board.undo_move(i, player); 

            let is_better = match player {
                Player::X => score > best_score,
                Player::O => score < best_score,
            };

            if is_better {
                best_score = score;
                best_move = i;
            }  
        }
        (best_score, best_move.0, best_move.1)
    }
}
