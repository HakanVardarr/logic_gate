use super::And;
use super::Not;
use super::Or;
use super::Signal;

pub struct Mux;

impl Mux {
    pub fn send_signal(sellect: &Signal, input1: &Signal, input2: &Signal) -> Signal {
        Or::send_signal(
            &And::send_signal(&Not::send_signal(sellect), input1),
            &And::send_signal(sellect, input2),
        )
    }
}
