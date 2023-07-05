fn number_to_string(i: i32) -> String {
    i.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_to_string() {
        assert_eq!(number_to_string(12345), "12345");
        assert_eq!(number_to_string(-12345), "-12345");
        assert_eq!(number_to_string(0), "0");
    }
}