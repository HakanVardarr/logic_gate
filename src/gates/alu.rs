use super::{Adder, And, Mux4Way, Or, Signal, Xor};

pub struct Alu;

impl Alu {
    pub fn send_signal(a: &Signal, b: &Signal, s0: &Signal, s1: &Signal) -> Signal {
        let (s, _) = Adder::send_signal(&Signal::Zero, a, b);
        Mux4Way::send_signal(
            &s,
            &And::send_signal(a, b),
            &Or::send_signal(a, b),
            &Xor::send_signal(a, b),
            s0,
            s1,
        )
    }
}
