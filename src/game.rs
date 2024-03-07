use pheap::PairingHeap;
use rand::Rng;
use nalgebra::Matrix4;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone)]
pub enum Move
{
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy)]
pub struct Board
{
    board: Matrix4<u8>,
}

impl Board
{
    pub fn new() -> Board
    {
        return Self::prepared_board();
    }

    pub fn get_board(&self) -> Matrix4<u8>
    {
        self.board
    }

    pub fn move_tile(&self, m: &Move) -> Option<Board>
    {
        let (i, j) = self.free_slot();
        let mut moved_board = self.board.clone();
        match m
        {
            Move::Down => {
                if i == 3
                {
                    return None;
                }
                moved_board[(i, j)] = moved_board[(i+1, j)];
                moved_board[(i+1, j)] = 0;
            },
            Move::Up => {
                if i == 0
                {
                    return None;
                }
                moved_board[(i, j)] = moved_board[(i-1, j)];
                moved_board[(i-1, j)] = 0;
            },
            Move::Right => {
                if j == 3
                {
                    return None;
                }
                moved_board[(i, j)] = moved_board[(i, j+1)];
                moved_board[(i, j+1)] = 0;
            },
            Move::Left => {
                if j == 0
                {
                    return None;
                }
                moved_board[(i, j)] = moved_board[(i, j-1)];
                moved_board[(i, j-1)] = 0;
            }
        }
        Option::Some(Board{board: moved_board})
    }

    fn free_slot(&self) -> (usize,usize)
    {
        for i in 0..4
        {
            for j in 0..4
            {
                if self.board[(i, j)] == 0
                {
                    return (i, j);
                }
            }
        }
        panic!("No free slot found");
    }

    // most of random board are unsolvable
    #[allow(dead_code)]
    fn random_board() -> Matrix4<u8>
    {
        let mut board = Matrix4::<u8>::zeros();
        for i in 1..16
        {
            loop {
                let v = rand::thread_rng().gen::<u8>() % 16;
                let x = (v / 4) as usize;
                let y = (v % 4) as usize;
                
                if board[(x, y)] == 0
                {
                    board[(x, y)] = i;
                    break;
                }
            }
        }
        board
    }

    fn prepared_board() -> Board
    {
        let mut board = Matrix4::<u8>::zeros();
        for i in 1..16
        {
            let v = i as usize - 1;
            board[(v/4, v%4)] = i;
        }
        let mut board = Board {board};

        let moves = Move::iter().collect::<Vec<Move>>();
        for _ in 0..100
        {
            
            let m = moves[rand::thread_rng().gen::<usize>() % 4].clone();
            if let Some(b) = board.move_tile(&m)
            {
                board = b;
            }
        }
        board
    }

}

pub struct BoardSolver
{}

struct HistoryNode
{
    board: Board,
    path: Vec<Move>
}

impl BoardSolver
{
    pub fn solve(board: &Board) -> Vec<Move>
    {
        let mut heap = PairingHeap::<HistoryNode,usize>::new();

        let root = HistoryNode{board: (*board).clone(), path: Vec::new()};
        heap.insert(root, Self::heuristic(board));

        while !heap.is_empty()
        {
            let (node, _) = heap.delete_min().unwrap();

            if Self::check_board(&node.board)
            {
                println!("Solved in {} moves", node.path.len());
                return node.path;
            }

            for dir in Move::iter()
            {
                if let Some(new_board) = node.board.move_tile(&dir)
                {
                    let heuristic = Self::heuristic_shortest_path(&new_board);
                    let mut path = node.path.clone();
                    path.push(dir);
                    let new_node = HistoryNode{board: new_board,  path};
                    heap.insert(new_node, node.path.len() + 1 + heuristic);
                }
            }

        }
        Vec::new()
    }

    fn heuristic(board: &Board) -> usize
    {
        let mut hit = 15;
        let board = board.get_board();
        for i in 1..16 {
            let v = i - 1;
            if board[(v/4, v%4)] != i as u8 {
                hit -= 1;
            }
        }
        hit
    }

    fn heuristic_shortest_path(board: &Board) -> usize
    {
        let board = board.get_board();
        let mut sum = 0;
        for i in 0..4
        {
            for j in 0..4
            {
                if board[(i, j)] != 0
                {
                    let appropriate = Self::appropriate_slot(board[(i, j)]);
                    sum += i.abs_diff(appropriate.0) + j.abs_diff(appropriate.1);
                }
            }
        }
        sum
    }

    fn appropriate_slot(value: u8) -> (usize, usize)
    {
        let value = value - 1;
        ((value as usize) / 4, (value as usize) % 4)
    }

    fn check_board(board: &Board) -> bool
    {
        let board = board.get_board();
        for i in 1..16 
        {
            let v = i - 1;
            if board[(v/4, v%4)] != i as u8 {
                return false;
            }
        }
        true
    }

}

