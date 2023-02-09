use super::And;
use super::Not;
use super::Signal;

pub struct Nand;

impl Nand {
    pub fn send_signal(input1: &Signal, input2: &Signal) -> Signal {
        Not::send_signal(&And::send_signal(input1, input2))
    }
}
