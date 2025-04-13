pub struct Guess {
    pub value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            // 予想の値は1から100の範囲でなければなりませんが、{}でした
            return Err(format!(
                "予想の値は1から100の範囲でなければなりませんが、{}でした",
                value
            ));
        }
        Ok(Guess { value })
    }
}

#[cfg(test)]
mod tests {
    use super::Guess;

    #[test]
    fn valid_guess() {
        let guess = Guess::new(50);
        assert!(guess.is_ok());
        assert_eq!(guess.unwrap().value, 50);
        let guess = Guess::new(1);
        assert!(guess.is_ok());
        assert_eq!(guess.unwrap().value, 1);
        let guess = Guess::new(100);
        assert!(guess.is_ok());
        assert_eq!(guess.unwrap().value, 100);
    }
    #[test]
    fn invalid_guess() {
        let guess = Guess::new(101);
        assert!(guess.is_err());
        let guess = Guess::new(0);
        assert!(guess.is_err());
    }
}
