pub struct Response {
    sequence: u32,
}

impl Response {
    pub fn new(sequence: u32) -> Self {
        Response { sequence: sequence }
    }

    pub fn sequence(&self) -> u32 {
        self.sequence
    }
}
