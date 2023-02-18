use super::{And, Not, Or, Signal};

pub struct Mux;

impl Mux {
    pub fn send_signal(s: &Signal, a: &Signal, b: &Signal) -> Signal {
        Or::send_signal(
            &And::send_signal(&Not::send_signal(s), a),
            &And::send_signal(s, b),
        )
    }
}
