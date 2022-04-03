#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Digit {
    D1 = 1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
}
pub type Idx = (Digit, Digit);
pub struct Square {
    first_row: u8,
    first_col: u8,
    cur: u8,
}
impl Square {
    fn new(num: Digit) -> Self {
        let d = u8::from(num) - 1;
        let first_row = (d / 3) * 3 + 1;
        let first_col = (d % 3) * 3 + 1;
        Self {
            first_row,
            first_col,
            cur: 0,
        }
    }
}
impl Iterator for Square {
    type Item = Idx;
    fn next(&mut self) -> Option<Self::Item> {
        let row = self.first_row + self.cur / 3;
        let col = self.first_col + self.cur % 3;
        self.cur += 1;
        if self.cur <= 9 {
            Some((row.try_into().unwrap(), col.try_into().unwrap()))
        } else {
            None
        }
    }
}
impl Digit {
    pub fn all() -> &'static [Self] {
        use Digit::*;
        static ALL: [Digit; 9] = [D1, D2, D3, D4, D5, D6, D7, D8, D9];
        &ALL
    }
    pub fn all_indices() -> Vec<Idx> {
        let mut v = Vec::with_capacity(81);
        for &l in Self::all() {
            for &c in Self::all() {
                v.push((l, c));
            }
        }
        v
    }
    pub fn row(self) -> impl Iterator<Item = Idx> {
        Self::all().iter().map(move |&col| (self, col))
    }
    pub fn col(self) -> impl Iterator<Item = Idx> {
        Self::all().iter().map(move |&row| (row, self))
    }
    pub fn square(self) -> Square {
        Square::new(self)
    }
    pub fn square_of(row: Digit, col: Digit) -> Self {
        let row = u8::from(row);
        let col = u8::from(col);
        match (row, col) {
            (1..=3, 1..=3) => Self::D1,
            (1..=3, 4..=6) => Self::D2,
            (1..=3, 7..=9) => Self::D3,
            (4..=6, 1..=3) => Self::D4,
            (4..=6, 4..=6) => Self::D5,
            (4..=6, 7..=9) => Self::D6,
            (7..=9, 1..=3) => Self::D7,
            (7..=9, 4..=6) => Self::D8,
            (7..=9, 7..=9) => Self::D9,
            (r, c) => unreachable!("({}, {})", r, c),
        }
    }
}
impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        u8::from(*self).fmt(f)
    }
}
impl From<Digit> for u8 {
    fn from(d: Digit) -> Self {
        unsafe { std::mem::transmute(d) }
    }
}
#[derive(Debug)]
pub struct U8ToDigitConversionError(u8);
impl TryFrom<u8> for Digit {
    type Error = U8ToDigitConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v @ 1..=9 => Ok(unsafe { std::mem::transmute(v) }),
            other => Err(U8ToDigitConversionError(other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move(Idx, Digit);
impl From<(Idx, Digit)> for Move {
    fn from((idx, d): (Idx, Digit)) -> Self {
        Self(idx, d)
    }
}
impl Move {
    pub fn pos(&self) -> Idx {
        self.0
    }
    pub fn digit(&self) -> Digit {
        self.1
    }
}

pub struct Sudoku([Option<Digit>; 81]);
impl Default for Sudoku {
    fn default() -> Self {
        Self([None; 81])
    }
}
impl std::ops::Index<Idx> for Sudoku {
    type Output = Option<Digit>;
    fn index(&self, index: Idx) -> &Self::Output {
        let row: usize = (u8::from(index.0) - 1).into();
        let col: usize = (u8::from(index.1) - 1).into();
        &self.0[row * 9 + col]
    }
}
impl std::ops::IndexMut<Idx> for Sudoku {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        let row: usize = (u8::from(index.0) - 1).into();
        let col: usize = (u8::from(index.1) - 1).into();
        &mut self.0[row * 9 + col]
    }
}
impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+-----+-----+-----+")?;
        for &l in Digit::all() {
            match l {
                Digit::D4 | Digit::D7 => {
                    write!(f, "+-----+-----+-----+\n|")?;
                }
                _ => write!(f, "|")?,
            }
            for &c in Digit::all() {
                let idx = (l, c);
                let sep = match c {
                    Digit::D3 | Digit::D6 | Digit::D9 => "|",
                    _ => " ",
                };
                if let Some(d) = self[idx] {
                    write!(f, "{}{}", d, sep)?;
                } else {
                    write!(f, " {}", sep)?;
                };
            }
            write!(f, "\n")?;
        }
        writeln!(f, "+-----+-----+-----+")
    }
}
#[derive(Debug)]
pub enum MoveError {
    // When it would make an invalid sudoku
    Invalid,
    // When there is a digit already
    NonEmpty,
}
impl Sudoku {
    pub fn new(array: [Option<Digit>; 81]) -> Self {
        Self(array)
    }
    pub fn row(&self, num: Digit) -> impl Iterator<Item = Digit> + '_ {
        num.row().map(|idx| self[idx].clone()).flatten()
    }
    pub fn col(&self, num: Digit) -> impl Iterator<Item = Digit> + '_ {
        num.col().map(|idx| self[idx].clone()).flatten()
    }
    pub fn square(&self, num: Digit) -> impl Iterator<Item = Digit> + '_ {
        num.square().map(|idx| self[idx].clone()).flatten()
    }
    pub fn possible_moves(&self, idx: Idx) -> PossibleMoves {
        PossibleMoves::new(self, idx)
    }
    pub fn empty_indices(&self) -> EmptyIndices<'_> {
        EmptyIndices::new(self)
    }
    pub fn play(&mut self, Move(idx, digit): Move) -> Result<(), MoveError> {
        if self[idx].is_some() {
            Err(MoveError::NonEmpty)
        } else {
            let moves: Vec<_> = self.possible_moves(idx).map(|m| m.digit()).collect();
            if !moves.contains(&digit) {
                Err(MoveError::Invalid)
            } else {
                self[idx] = Some(digit);
                Ok(())
            }
        }
    }
}

pub struct PossibleMoves {
    idx: Idx,
    f: Vec<Digit>,
}
impl PossibleMoves {
    fn new(sudoku: &Sudoku, idx: Idx) -> Self {
        let (row, col) = idx;
        let mut f = Vec::from(Digit::all());
        let mut a = Vec::with_capacity(9);
        for d in sudoku
            .row(row)
            .chain(sudoku.col(col))
            .chain(sudoku.square(Digit::square_of(row, col)))
        {
            if !a.contains(&d) {
                a.push(d);
            }
        }
        f.retain(|digit| !a.contains(digit));
        Self { idx, f }
    }
}
impl Iterator for PossibleMoves {
    type Item = Move;
    fn next(&mut self) -> Option<Move> {
        self.f.pop().map(|d| (self.idx, d).into())
    }
}

pub struct EmptyIndices<'s> {
    sudoku: &'s Sudoku,
    row: u8,
    col: u8,
}
impl<'s> EmptyIndices<'s> {
    fn new(sudoku: &'s Sudoku) -> Self {
        Self {
            sudoku,
            row: 1,
            col: 1,
        }
    }
    fn next_idx(&mut self) -> Option<Idx> {
        let next = (self.row.try_into().ok()?, self.col.try_into().ok()?);
        if self.col < 9 {
            self.col += 1;
        } else if self.row <= 9 {
            self.col = 1;
            self.row += 1;
        }
        Some(next)
    }
}
impl<'s> Iterator for EmptyIndices<'s> {
    type Item = Idx;
    fn next(&mut self) -> Option<Self::Item> {
        let mut idx = self.next_idx()?;
        loop {
            if self.sudoku[idx].is_none() {
                break Some(idx);
            }
            idx = self.next_idx()?;
        }
    }
}

#[macro_export]
macro_rules! sudoku {
    [$($i:expr)+] => {
        $crate::sudoku::Sudoku::new([$(
            match $i {
                0 => None,
                o => Some(o.try_into().unwrap())
            },
        )+])
    };
}

#[macro_export]
macro_rules! smove {
    ($r:expr, $c:expr; $d:expr) => {
        (
            ($r.try_into().unwrap(), $c.try_into().unwrap()),
            $d.try_into().unwrap(),
        )
            .into()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_works() {
        use Digit::*;
        let square_1: Vec<_> = D1.square().collect();
        assert_eq!(
            square_1,
            &[
                (D1, D1),
                (D1, D2),
                (D1, D3),
                (D2, D1),
                (D2, D2),
                (D2, D3),
                (D3, D1),
                (D3, D2),
                (D3, D3)
            ]
        );
        let square_2: Vec<_> = D2.square().collect();
        assert_eq!(
            square_2,
            &[
                (D1, D4),
                (D1, D5),
                (D1, D6),
                (D2, D4),
                (D2, D5),
                (D2, D6),
                (D3, D4),
                (D3, D5),
                (D3, D6)
            ]
        );
        let square_3: Vec<_> = D3.square().collect();
        assert_eq!(
            square_3,
            &[
                (D1, D7),
                (D1, D8),
                (D1, D9),
                (D2, D7),
                (D2, D8),
                (D2, D9),
                (D3, D7),
                (D3, D8),
                (D3, D9)
            ]
        );
        let square_4: Vec<_> = D4.square().collect();
        assert_eq!(
            square_4,
            &[
                (D4, D1),
                (D4, D2),
                (D4, D3),
                (D5, D1),
                (D5, D2),
                (D5, D3),
                (D6, D1),
                (D6, D2),
                (D6, D3)
            ]
        );
        let square_5: Vec<_> = D5.square().collect();
        assert_eq!(
            square_5,
            &[
                (D4, D4),
                (D4, D5),
                (D4, D6),
                (D5, D4),
                (D5, D5),
                (D5, D6),
                (D6, D4),
                (D6, D5),
                (D6, D6)
            ]
        );
        let square_6: Vec<_> = D6.square().collect();
        assert_eq!(
            square_6,
            &[
                (D4, D7),
                (D4, D8),
                (D4, D9),
                (D5, D7),
                (D5, D8),
                (D5, D9),
                (D6, D7),
                (D6, D8),
                (D6, D9)
            ]
        );
        let square_7: Vec<_> = D7.square().collect();
        assert_eq!(
            square_7,
            &[
                (D7, D1),
                (D7, D2),
                (D7, D3),
                (D8, D1),
                (D8, D2),
                (D8, D3),
                (D9, D1),
                (D9, D2),
                (D9, D3)
            ]
        );
        let square_8: Vec<_> = D8.square().collect();
        assert_eq!(
            square_8,
            &[
                (D7, D4),
                (D7, D5),
                (D7, D6),
                (D8, D4),
                (D8, D5),
                (D8, D6),
                (D9, D4),
                (D9, D5),
                (D9, D6)
            ]
        );
        let square_9: Vec<_> = D9.square().collect();
        assert_eq!(
            square_9,
            &[
                (D7, D7),
                (D7, D8),
                (D7, D9),
                (D8, D7),
                (D8, D8),
                (D8, D9),
                (D9, D7),
                (D9, D8),
                (D9, D9)
            ]
        );
    }

    #[test]
    fn sudoku_moves() {
        let mut sudoku = sudoku![
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

        assert!(sudoku.play(smove!(1,3;1)).is_ok());
        assert!(matches!(
            sudoku.play(smove!(1,3;1)),
            Err(MoveError::NonEmpty)
        ));
        assert!(matches!(
            sudoku.play(smove!(1,4;1)),
            Err(MoveError::Invalid)
        ));
    }

    #[test]
    fn empty_indices_empty() {
        let sudoku = Sudoku::default();
        assert_eq!(sudoku.empty_indices().count(), 81);
    }
}
