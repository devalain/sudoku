mod sudoku;
use sudoku::{Idx, Move, MoveError, Sudoku};

struct Solution {
    sudoku: Sudoku,
    empty_indices: Vec<Idx>,
    moves: Vec<Idx>,
    path: Vec<usize>,
    next_move: usize,
}
impl Solution {
    fn push_move(&mut self, m: Move) -> Result<(), MoveError> {
        self.sudoku.play(m)?;
        self.moves.push(m.pos());
        self.path.push(self.next_move);
        self.next_move = 0;
        Ok(())
    }
    fn pop_move(&mut self) -> Option<Move> {
        if let Some(i) = self.moves.pop() {
            let d = self.sudoku[i].take().unwrap();
            let m = (i, d).into();
            self.next_move = self.path.pop().unwrap() + 1;
            Some(m)
        } else {
            None
        }
    }
    fn next_empty_index(&mut self) -> Option<Idx> {
        self.empty_indices.iter().copied().nth(self.moves.len())
    }
    fn next_possible_move(&mut self, idx: Idx) -> Option<Move> {
        self.sudoku.possible_moves(idx).nth(self.next_move)
    }
}

fn solve(sudoku: Sudoku) -> Option<Sudoku> {
    let empty_indices: Vec<_> = sudoku.empty_indices().collect();
    let mut solution = Solution {
        sudoku,
        empty_indices,
        moves: Vec::new(),
        path: Vec::new(),
        next_move: 0
    };
    if solve_r(&mut solution) {
        Some(solution.sudoku)
    } else {
        None
    }
}
fn solve_r(sol: &mut Solution) -> bool {
    let mut solved = false;
    loop {
        display(&sol.sudoku);
        if let Some(idx) = sol.next_empty_index() {
            if let Some(possible_move) = sol.next_possible_move(idx) {
                sol.push_move(possible_move).unwrap();
                if sol.sudoku.empty_indices().count() > 0 {
                    solved = solve_r(sol);
                    if !solved {
                        let _ = sol.pop_move();
                    }
                } else {
                    solved = true;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        if solved {
            break;
        }
    }
    solved
}

fn display(sudoku: &Sudoku) {
    std::thread::sleep(std::time::Duration::from_millis(10));
    print!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), sudoku);
}

fn main() {
    let sudoku = sudoku![
        5 3 0 0 7 0 0 0 0
        6 0 0 1 9 5 0 0 0
        0 9 8 0 0 0 0 6 0
        8 0 0 0 6 0 0 0 3
        4 0 0 8 0 3 0 0 1
        7 0 0 0 2 0 0 0 6
        0 6 0 0 0 0 2 8 0
        0 0 0 4 1 9 0 0 5
        0 0 0 0 8 0 0 7 9
    ];
    if let Some(sudoku) = solve(sudoku) {
        display(&sudoku);
    }
}
