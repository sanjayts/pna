#[cfg(test)]
mod tests {
    use crate::models::{random_moves, Move};

    #[test]
    fn test_ser_der() {
        let m1 = Move::Down(11);
        let out = serde_json::to_vec(&m1).unwrap();
        let m2 = serde_json::from_slice::<Move>(&out).unwrap();
        assert_eq!(m1, m2);

        let str_repr = String::from_utf8(out.clone()).unwrap();
        assert_eq!(r#"{"Down":11}"#, str_repr);
        assert_eq!(11, out.len());
    }

    #[test]
    fn test_bulk_ser_der() {
        let moves = random_moves(1000);
        let bytes = serde_json::to_vec(&moves).unwrap();

        assert_eq!(10891, bytes.len());
    }
}
