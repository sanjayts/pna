#[cfg(test)]
mod tests {
    use crate::models::{random_moves, Move};
    use std::collections::HashMap;

    #[test]
    fn test_ser_der() {
        let m1 = Move::Up(42);
        let bytes = bson::to_vec(&m1).unwrap();
        let m2 = bson::from_slice::<Move>(&bytes).unwrap();
        assert_eq!(m1, m2);
        assert_eq!(17, bytes.len());
    }

    #[test]
    fn test_bulk_ser_der() {
        let moves = random_moves(1000);
        let mut map = HashMap::with_capacity(1);
        map.insert("moves", &moves);

        let bytes = bson::to_vec(&map).unwrap();
        let new_map: HashMap<String, Vec<Move>> = bson::from_slice(&bytes).unwrap();

        assert_eq!(new_map.get("moves").unwrap(), &moves);
        assert_eq!(21907, bytes.len());
    }
}
