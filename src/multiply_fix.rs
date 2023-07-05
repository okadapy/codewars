//  Not Working!
// fn multiply(a:i32, b:i32) {
//     a * b
// }

fn multiply(a:i32, b:i32) -> i32 {
    a * b
}

// Do i really need to test that???
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 2), 4);
        assert_eq!(multiply(5, 5), 25);
        assert_eq!(multiply(10, 25), 250);
        assert_eq!(multiply(11, 11), 121);
    }
}