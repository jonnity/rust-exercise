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
