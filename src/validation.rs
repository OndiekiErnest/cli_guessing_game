pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            // Return an error if the value is out of range
            Err(format!("Value must be between 1 and 100, got {}", value))
        } else {
            // Return a valid Guess instance
            Ok(Guess { value })
        }
    }

    pub fn value(&self) -> u32 {
        return self.value;
    }
}