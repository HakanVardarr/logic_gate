use super::{Adder, FourBit, Signal};
pub struct FourBitAdder;

impl FourBitAdder {
    pub fn send_signal(c: &Signal, s: &Signal, b0: &FourBit, b1: &FourBit) -> (FourBit, Signal) {
        let temp = b1.reverse();
        let mut b2 = b1.clone();
        if s == &Signal::One {
            b2 = temp;
            if b2 != "1111".parse().unwrap() {
                let asadas: FourBit = "0001".parse().unwrap();
                let (s, _) = FourBitAdder::send_signal(&Signal::Zero, &Signal::Zero, &b2, &asadas);
                b2 = s;
            }
        }

        let (s1, c1) = Adder::send_signal(c, &b0.bit4, &b2.bit4);
        let (s2, c2) = Adder::send_signal(&c1, &b0.bit3, &b2.bit3);
        let (s3, c3) = Adder::send_signal(&c2, &b0.bit2, &b2.bit2);
        let (s4, c4) = Adder::send_signal(&c3, &b0.bit1, &b2.bit1);

        (
            FourBit {
                bit1: s4,
                bit2: s3,
                bit3: s2,
                bit4: s1,
            },
            c4,
        )
    }
}
