use super::{Not, Or, Signal};

pub struct Nor;

impl Nor {
    pub fn send_signal(a: &Signal, b: &Signal) -> Signal {
        Not::send_signal(&Or::send_signal(a, b))
    }
}
