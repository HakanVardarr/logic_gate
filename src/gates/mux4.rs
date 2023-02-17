use super::{And3, Not, Or, Signal};
pub struct Mux4;

impl Mux4 {
    pub fn send_signal(
        input1: &Signal,
        input2: &Signal,
        input3: &Signal,
        input4: &Signal,
        s1: &Signal,
        s2: &Signal,
    ) -> Signal {
        let s1i = &Not::send_signal(s1);
        let s2i = &Not::send_signal(s2);

        let and1 = &And3::send_signal(s1i, input1, s2i);
        let and2 = &And3::send_signal(s1, input2, s2i);
        let and3 = &And3::send_signal(s1i, input3, s2);
        let and4 = &And3::send_signal(s1, input4, s2);

        Or::send_signal(&Or::send_signal(and1, and2), &Or::send_signal(and3, and4))
    }
}
