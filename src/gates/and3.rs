use super::And;
use super::Signal;

pub struct And3;

impl And3 {
    pub fn send_signal(input1: &Signal, input2: &Signal, input3: &Signal) -> Signal {
        And::send_signal(input1, &And::send_signal(input3, input2))
    }
}
