mod game;
use game::{Board, BoardSolver};


fn main() 
{
    let mut b = Board::new();
    let path = BoardSolver::solve(&b);
    for m in path
    {
        b = b.move_tile(&m).unwrap();
        println!("{}", b.get_board());
    }
}   
