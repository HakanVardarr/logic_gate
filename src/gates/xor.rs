use super::Nand;
use super::Signal;

pub struct Xor;

impl Xor {
    pub fn send_signal(input1: &Signal, input2: &Signal) -> Signal {
        let helper_signal = Nand::send_signal(input1, input2);

        Nand::send_signal(
            &Nand::send_signal(input1, &helper_signal),
            &Nand::send_signal(input2, &helper_signal),
        )
    }
}
