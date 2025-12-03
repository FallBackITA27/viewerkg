pub struct FinishTime {
    minutes: u8, 			// 7 bits, offset 0x00
    seconds: u8,			// 7 bits, offset 0x00.7
    milliseconds: u16,		// 10 bits, offset 0x01.6
    string: String          // Total time as string
}

impl FinishTime {
    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    pub fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    pub fn string(&self) -> &str {
        &self.string
    }

    pub fn new(minutes: u8, seconds: u8, milliseconds: u16) -> Self {
        Self {
            minutes,
            seconds,
            milliseconds,
            string: format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds),
        }
    }
}
