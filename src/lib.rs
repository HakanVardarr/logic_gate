pub mod gates;
pub mod bytes;

use std::fmt;
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Signal {
    One,
    Zero,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Zero => write!(f, "0"),
        }
    }
}
