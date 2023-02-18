use super::Signal;

pub struct And;

impl And {
    pub fn send_signal(a: &Signal, b: &Signal) -> Signal {
        if a == &Signal::One && b == &Signal::One {
            Signal::One
        } else {
            Signal::Zero
        }
    }
}
