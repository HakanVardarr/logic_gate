pub struct Not;
use super::Signal;

impl Not {
    pub fn send_signal(input: &Signal) -> Signal {
        match input {
            Signal::One => Signal::Zero,
            Signal::Zero => Signal::One,
        }
    }
}
