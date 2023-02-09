pub struct Or;

use super::Nand;
use super::Signal;

impl Or {
    pub fn send_signal(input1: &Signal, input2: &Signal) -> Signal {
        Nand::send_signal(
            &Nand::send_signal(input1, input1),
            &Nand::send_signal(input2, input2),
        )
    }
}
