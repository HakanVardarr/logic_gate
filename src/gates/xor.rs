use super::And;
use super::Not;
use super::Or;
use super::Signal;

pub struct Xor;

impl Xor {
    pub fn send_signal(input1: &Signal, input2: &Signal) -> Signal {
        Or::send_signal(
            &And::send_signal(&Not::send_signal(input1), input2),
            &And::send_signal(input1, &Not::send_signal(input2)),
        )
    }
}
