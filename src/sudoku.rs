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
impl Digit {
    pub fn all() -> impl Iterator<Item = Self> {
        use Digit::*;
        vec![D1, D2, D3, D4, D5, D6, D7, D8, D9].into_iter()
    }
    pub fn all_indices() -> Vec<Idx> {
        let mut v = Vec::with_capacity(81);
        for l in Self::all() {
            for c in Self::all() {
                v.push((l, c));
            }
        }
        v
    }
    pub fn row(self) -> impl Iterator<Item = Idx> {
        Self::all().map(move |col| (self, col))
    }
    pub fn col(self) -> impl Iterator<Item = Idx> {
        Self::all().map(move |row| (row, self))
    }
    pub fn square(self) -> impl Iterator<Item = Idx> {
        let d = u8::from(self) - 1;
        let mut v = Vec::with_capacity(9);

        let first_row = (d / 3) * 3 + 1;
        let first_col = (d % 3) * 3 + 1;
        for n in 0..9 {
            let row = first_row + n / 3;
            let col = first_col + n % 3;
            v.push((row.try_into().unwrap(), col.try_into().unwrap()));
        }
        v.into_iter()
    }
    pub fn square_of(row: Digit, col: Digit) -> Self {
        let row = u8::from(row);
        let col = u8::from(col);
        match (row, col) {
            (1..=3, 1..=3) => Self::D1,
            (1..=3, 4..=6) => Self::D2,
            (1..=3, 7..=9) => Self::D3,
            (4..=5, 1..=3) => Self::D4,
            (4..=5, 4..=6) => Self::D5,
            (4..=5, 7..=9) => Self::D6,
            (7..=9, 1..=3) => Self::D7,
            (7..=9, 4..=6) => Self::D8,
            (7..=9, 7..=9) => Self::D9,
            _ => unreachable!(),
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

#[derive(Debug)]
pub struct Move(Idx, Digit);
impl From<(Idx, Digit)> for Move {
    fn from((idx, d): (Idx, Digit)) -> Self {
        Self(idx, d)
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
        for l in Digit::all() {
            match l {
                Digit::D4 | Digit::D7 => {
                    write!(f, "+-----+-----+-----+\n|")?;
                }
                _ => write!(f, "|")?,
            }
            for c in Digit::all() {
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
    pub fn row(&self, num: Digit) -> impl Iterator<Item = Option<Digit>> + '_ {
        num.row().map(|idx| self[idx].clone())
    }
    pub fn col(&self, num: Digit) -> impl Iterator<Item = Option<Digit>> + '_ {
        num.col().map(|idx| self[idx].clone())
    }
    pub fn square(&self, num: Digit) -> impl Iterator<Item = Option<Digit>> + '_ {
        num.square().map(|idx| self[idx].clone())
    }
    pub fn play(&mut self, Move(idx, digit): Move) -> Result<(), MoveError> {
        if self[idx].is_some() {
            Err(MoveError::NonEmpty)
        } else {
            let (row, col) = idx;
            let square_num = Digit::square_of(row, col);
            let row_digits: Vec<_> = self.row(row).flatten().collect();
            let col_digits: Vec<_> = self.col(col).flatten().collect();
            let square_digits: Vec<_> = self.square(square_num).flatten().collect();
            if row_digits.contains(&digit)
                || col_digits.contains(&digit)
                || square_digits.contains(&digit)
            {
                Err(MoveError::Invalid)
            } else {
                self[idx] = Some(digit);
                Ok(())
            }
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
        ).into()
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
}
