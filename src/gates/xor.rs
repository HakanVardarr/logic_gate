use super::{And, Not, Or, Signal};

pub struct Xor;

impl Xor {
    pub fn send_signal(a: &Signal, b: &Signal) -> Signal {
        Or::send_signal(
            &And::send_signal(a, &Not::send_signal(b)),
            &And::send_signal(&Not::send_signal(a), b),
        )
    }
}
