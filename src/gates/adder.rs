use super::And;
use super::Or;
use super::Signal;
use super::Xor;

pub struct Adder;

impl Adder {
    pub fn send_signal(carry: &Signal, input1: &Signal, input2: &Signal) -> (Signal, Signal) {
        let helper_signal = Xor::send_signal(input1, input2);
        (
            Xor::send_signal(&helper_signal, carry),
            Or::send_signal(
                &And::send_signal(&helper_signal, carry),
                &And::send_signal(input1, input2),
            ),
        )
    }
}
