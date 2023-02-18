use super::{Nand, Signal};
pub struct Or;

impl Or {
    pub fn send_signal(a: &Signal, b: &Signal) -> Signal {
        Nand::send_signal(&Nand::send_signal(a, a), &Nand::send_signal(b, b))
    }
}
