pub struct FourBitAdder;
use super::Adder;
use super::FourBit;
use super::Signal;

impl FourBitAdder {
    pub fn send_signal(
        carry: &Signal,
        bits1: &FourBit,
        bits2: &FourBit,
    ) -> Result<(FourBit, Signal), String> {
        let (sum1, carry1) = Adder::send_signal(carry, &bits1.bit4, &bits2.bit4);
        let (sum2, carry2) = Adder::send_signal(&carry1, &bits1.bit3, &bits2.bit3);
        let (sum3, carry3) = Adder::send_signal(&carry2, &bits1.bit2, &bits2.bit2);
        let (sum4, carry4) = Adder::send_signal(&carry3, &bits1.bit1, &bits2.bit1);
        if carry4 == Signal::One {
            return Err(String::from("4 bit overflow"));
        }
        Ok((
            FourBit {
                bit1: sum4,
                bit2: sum3,
                bit3: sum2,
                bit4: sum1,
            },
            carry4,
        ))
    }
}
