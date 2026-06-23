use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct BasicAgent {}

// Put your solution here.
impl Agent for BasicAgent {    
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let remaining = board.moves().len();
        let max_depth = if remaining <= 9 { remaining as u32 } else { 4 };
        BasicAgent::minimax(board, player, 0, max_depth)
    }
}

impl BasicAgent {
    fn heuristic(board: &Board) -> i32 {
        board.score()
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
            
            let (score, _, _) = BasicAgent::minimax(board, next_player, depth + 1, max_depth);  

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