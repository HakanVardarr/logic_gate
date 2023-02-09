pub struct And;
use super::Signal;

impl And {
    pub fn send_signal(input1: &Signal, input2: &Signal) -> Signal {
        if input1 == &Signal::One && input2 == &Signal::One {
            Signal::One
        } else {
            Signal::Zero
        }
    }
}
