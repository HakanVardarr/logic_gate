use super::Signal;

/// And Gate
/// # Usage
///
/// ```
///     use::logic_gate::gates::And;
///     use::logic_gate::Signal;
///     
///     let output = And::send_signal(&Signal::One, &Signal::One);
///     println!("{output}")
/// ```
///
/// # Truth Table
/// A B OUT
/// 1 1 1
/// 1 0 0
/// 0 1 0
/// 0 0 0
pub struct And;

impl And {
    pub fn send_signal(input1: &Signal, input2: &Signal) -> Signal {
        if input1 == &Signal::One && input2 == &Signal::One {
            Signal::One
        } else {
            Signal::Zero
        }
    }
}
