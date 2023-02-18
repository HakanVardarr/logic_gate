use super::{And3Way, Not, Or, Signal};
pub struct Mux4Way;

impl Mux4Way {
    pub fn send_signal(
        a: &Signal,
        b: &Signal,
        c: &Signal,
        d: &Signal,
        s0: &Signal,
        s1: &Signal,
    ) -> Signal {
        let s0i = &Not::send_signal(s0);
        let s1i = &Not::send_signal(s1);

        Or::send_signal(
            &Or::send_signal(
                &And3Way::send_signal(s0i, a, s1i),
                &And3Way::send_signal(s0, b, s1i),
            ),
            &Or::send_signal(
                &And3Way::send_signal(s0i, c, s1),
                &And3Way::send_signal(s0, d, s1),
            ),
        )
    }
}
