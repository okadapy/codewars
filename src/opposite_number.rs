fn opposite(number: i32) -> i32 {
    -number
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_opposite() {
        assert_eq!(opposite(1), -1);
        assert_eq!(opposite(-2), 2);
    }
}
