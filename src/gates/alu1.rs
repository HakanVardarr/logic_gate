use super::{Adder, And, Mux4, Or, Signal, Xor};

pub struct Alu1;

impl Alu1 {
    pub fn send_signal(input1: &Signal, input2: &Signal, s1: &Signal, s2: &Signal) -> Signal {
        let (sum, _) = Adder::send_signal(&Signal::Zero, input1, input2);
        Mux4::send_signal(
            &sum,
            &And::send_signal(input1, input2),
            &Or::send_signal(input1, input2),
            &Xor::send_signal(input1, input2),
            s1,
            s2,
        )
    }
}
