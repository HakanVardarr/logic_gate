use super::{And, Or, Signal, Xor};
pub struct Adder;

impl Adder {
    pub fn send_signal(c: &Signal, a: &Signal, b: &Signal) -> (Signal, Signal) {
        let h = &Xor::send_signal(a, b);
        (
            Xor::send_signal(h, c),
            Or::send_signal(&And::send_signal(h, c), &And::send_signal(a, b)),
        )
    }
}
