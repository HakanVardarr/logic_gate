use super::Nand;
use super::Signal;

pub struct Mux;

impl Mux {
    pub fn send_signal(sellect: &Signal, input1: &Signal, input2: &Signal) -> Signal {
        Nand::send_signal(
            &Nand::send_signal(input1, &Nand::send_signal(sellect, sellect)),
            &Nand::send_signal(input2, sellect),
        )
    }
}
