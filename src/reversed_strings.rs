fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reversed_string() {
        assert_eq!(solution("dcba"), "abcd");
        assert_eq!(solution("!dlrow olleh"), "hello world!");
        assert_eq!(solution("0987654321"), "1234567890");
    }
}
