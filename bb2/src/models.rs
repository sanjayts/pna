#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Move {
    Up(u64),
    Down(u64),
    Left(u64),
    Right(u64),
}

#[cfg(test)]
pub fn random_moves(count: u64) -> Vec<Move> {
    let mut moves = vec![];
    for i in 0..count {
        moves.push(Move::Up(i));
    }
    moves
}
