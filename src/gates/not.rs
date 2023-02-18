pub struct Not;
use super::Signal;

impl Not {
    pub fn send_signal(a: &Signal) -> Signal {
        match a {
            Signal::One => Signal::Zero,
            Signal::Zero => Signal::One,
        }
    }
}
