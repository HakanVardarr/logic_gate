use super::{And, Not};
use crate::Signal;

pub struct Demux;

impl Demux {
    pub fn send_signal(s: &Signal, d: &Signal) -> (Signal, Signal) {
        (
            And::send_signal(s, &Not::send_signal(d)),
            And::send_signal(s, d),
        )
    }
}
