fn greet(name: &str, owner: &str) -> String {
    match name.eq(owner) {
        true => "Hello boss".to_string(),
        false => "Hello guest".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Daniel", "Daniel"), "Hello boss");
        assert_eq!(greet("Greg", "Daniel"), "Hello guest");
    }
}
