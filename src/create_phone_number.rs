fn create_phone_number(numbers: &[u8]) -> String {
    let mut buf = String::from("(");
    for i in numbers.iter() {
        if buf.len() == 4 {
            buf.push(')');
            buf.push(' ');
            buf.push_str(&i.to_string());
        } else if buf.len() == 9 {
            buf.push('-');
            buf.push_str(&i.to_string())
        } else {
            buf.push_str(&i.to_string())
        }
    }
    return buf;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "(123) 456-7890"
        );
        assert_eq!(
            create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            "(111) 111-1111"
        );
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
            "(123) 456-7899"
        );
    }
}
