use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // BASE CASE: game is already over 
        if board.game_over() {
            // returns final score and a placeholder move
            return (board.score(), 0, 0);
        }

        // RECURSIVE CASE
        
        // store all currently available moves 
        let moves = board.moves(); 

        // set initial best score
        // when acting as X (wants to maximize) -> best_score = -infinity 
        // when acting as O (wants to minimize) -> best_score = infinity 
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        }; 

        // create variable to store the best known move
        let mut best_move = moves[0]; // set it as placeholder move for now

        // iterate through all possible moves
        for i in moves {
            // apply the current move in moves as current player 
            // changes the board directly because board was passed through reference
            board.apply_move(i, player); 

            // define the next player based on current player
            let next_player = match player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            
            // call the solve function with the board (which is now altered by current move)
            // now, solve function acts as next player
            // only store the score from the returned tuple
            let (score, _, _) = SolutionAgent::solve(board, next_player, _time_limit);  

            // return board to original state so next iteration of the loop sees the original board
            board.undo_move(i, player); 

            // see if score of this move beats the best score so far
            let is_better = match player {
                Player::X => score > best_score,
                Player::O => score < best_score,
            };

            // store the move and new best score if old best is beaten
            if is_better {
                best_score = score;
                best_move = i;
            }  
        }
        (best_score, best_move.0, best_move.1)
    }

}
