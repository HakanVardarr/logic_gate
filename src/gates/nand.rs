use super::{And, Not, Signal};

pub struct Nand;

impl Nand {
    pub fn send_signal(a: &Signal, b: &Signal) -> Signal {
        Not::send_signal(&And::send_signal(a, b))
    }
}
