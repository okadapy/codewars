fn abbrev_name(name: &str) -> String {
    let n: Vec<&str> = name.split(" ").collect();
    format!(
        "{}.{}",
        n[0].chars().nth(0).unwrap(),
        n[1].chars().nth(0).unwrap()
    )
    .to_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(abbrev_name("Sam Harris"), "S.H");
        assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
        assert_eq!(abbrev_name("Evan Cole"), "E.C");
        assert_eq!(abbrev_name("P Favuzzi"), "P.F");
        assert_eq!(abbrev_name("David Mendieta"), "D.M");
    }
}
