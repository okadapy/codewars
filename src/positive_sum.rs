fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|x| x.is_positive()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_positive_sum() {
        assert_eq!(positive_sum(&[1, 2, 3]), 6);
        assert_eq!(positive_sum(&[1, -1, 2, -2, 3, -3]), 6);
        assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }

    #[test]
    fn test_all_negatives() {
        assert_eq!(positive_sum(&[-1, -2, -3, -4, -5]), 0);
    }
}
