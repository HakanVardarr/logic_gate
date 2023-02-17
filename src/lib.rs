pub mod bytes;
pub mod gates;
#[cfg(test)]
mod tests;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Signal {
    One,
    Zero,
}

impl Signal {
    fn reverse(&self) -> Self {
        match self {
            Self::One => Self::Zero,
            Self::Zero => Self::One,
        }
    }
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Zero => write!(f, "0"),
        }
    }
}
