use super::{Alu, FourBit, Signal};

pub struct Alu4Way;

impl Alu4Way {
    pub fn send_signal(a: &FourBit, b: &FourBit, s0: &Signal, s1: &Signal, c: &Signal) -> FourBit {
        let (bit1, carry1) = Alu::send_signal(&a.bit4, &b.bit4, s0, s1, c);
        let (bit2, carry2) = Alu::send_signal(&a.bit3, &b.bit3, s0, s1, &carry1);
        let (bit3, carry3) = Alu::send_signal(&a.bit2, &b.bit2, s0, s1, &carry2);
        let (bit4, _) = Alu::send_signal(&a.bit1, &b.bit1, s0, s1, &carry3);

        println!("{bit4} {bit3} {bit2} {bit1}");

        FourBit {
            bit1: bit4,
            bit2: bit3,
            bit3: bit2,
            bit4: bit1,
        }
    }
}
