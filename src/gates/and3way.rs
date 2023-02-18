use super::{And, Signal};

pub struct And3Way;

impl And3Way {
    pub fn send_signal(a: &Signal, b: &Signal, c: &Signal) -> Signal {
        And::send_signal(a, &And::send_signal(b, c))
    }
}
