// i think this is how it works

#![allow(dead_code)]
#![allow(unused_variables)]

pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth
}

pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

pub enum PieceValue {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

pub struct Board {
    
}

pub struct Row {
    spaces: [Space; 8]
}

pub struct Space {
    rank: Rank,
    file: File,
    piece: Option<Piece>,
}

pub struct Piece {
    value: PieceValue
}

impl Board {

    pub fn new() -> Self {
        // TEMP \/
        let board: Board = Board::new();
        board
    }

}

impl Row {

    pub fn new(Rank rank) -> Self {
        // TEMP \/
        let row: Row;
        match rank {
            First => row = Row::from_str("RNBQKBNR"),
            Second => row = Row::from_str("PPPPPPPP"),
            Third => row = Row::empty(),
            Fourth => row = Row::empty(),
            Fifth => row = Row::empty(),
            Sixth => row = Row::empty(),
            Seventh => row = Row::from_str("pppppppp"),
            Eighth => row = Row::from_str("rnbkqbnr")
        }
        row
    }

    pub fn from_str(&str string) -> Self {
        let row: Row;
        for i in 0..=8 {
            row.spaces[i] = 
        }
    }

}
