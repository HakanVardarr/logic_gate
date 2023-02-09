use super::Signal;
use std::str::FromStr;

#[derive(Debug)]
pub struct FourBit {
    pub bit1: Signal,
    pub bit2: Signal,
    pub bit3: Signal,
    pub bit4: Signal,
}

impl FourBit {
    pub fn convert(&self) -> i32 {
        let mut number = 0;
        if self.bit1 == Signal::One {
            number += 8;
        }
        if self.bit2 == Signal::One {
            number += 4;
        }
        if self.bit3 == Signal::One {
            number += 2;
        }
        if self.bit4 == Signal::One {
            number += 1;
        }

        number
    }
}

#[derive(Debug)]
pub struct ParseFourBitError;

impl FromStr for FourBit {
    type Err = ParseFourBitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut byte = FourBit {
            bit1: Signal::Zero,
            bit2: Signal::Zero,
            bit3: Signal::Zero,
            bit4: Signal::Zero,
        };

        if s.len() != 4 {
            return Err(ParseFourBitError);
        }

        for (i, c) in s.chars().enumerate() {
            match c {
                '1' => {
                    if i == 0 {
                        byte.bit1 = Signal::One;
                    } else if i == 1 {
                        byte.bit2 = Signal::One;
                    } else if i == 2 {
                        byte.bit3 = Signal::One;
                    } else if i == 3 {
                        byte.bit4 = Signal::One;
                    }
                }
                '0' => {
                    if i == 0 {
                        byte.bit1 = Signal::Zero;
                    } else if i == 1 {
                        byte.bit2 = Signal::Zero;
                    } else if i == 2 {
                        byte.bit3 = Signal::Zero;
                    } else if i == 3 {
                        byte.bit4 = Signal::Zero;
                    }
                }
                _ => (),
            }
        }

        Ok(byte)
    }
}
