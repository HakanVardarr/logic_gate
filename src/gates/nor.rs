use super::Not;
use super::Or;
use super::Signal;

pub struct Nor;

impl Nor {
    pub fn send_signal(input1: &Signal, input2: &Signal) -> Signal {
        Not::send_signal(&Or::send_signal(input1, input2))
    }
}
