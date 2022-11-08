#[cfg(test)]
mod tests {
    use crate::models::{random_moves, Move};

    #[test]
    fn test_ser_der() {
        let m1 = Move::Up(42);
        let str = ron::to_string(&m1).unwrap();
        let m2 = ron::from_str::<Move>(&str).unwrap();

        assert_eq!(m1, m2);
        assert_eq!(6, str.len());
        assert_eq!("Up(42)", str);
    }

    #[test]
    fn test_bulk_ser_der() {
        let moves = random_moves(1000);
        let str = ron::to_string(&moves).unwrap();
        let new_moves = ron::from_str::<Vec<Move>>(&str).unwrap();

        assert_eq!(moves, new_moves);
        assert_eq!(7891, str.len());
    }
}
