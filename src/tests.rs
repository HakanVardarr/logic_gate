use crate::bytes::FourBit;
use crate::Signal;

#[test]
fn fourbit_parse() -> Result<(), Box<dyn std::error::Error>> {
    let bit1 = FourBit {
        bit1: Signal::One,
        bit2: Signal::Zero,
        bit3: Signal::Zero,
        bit4: Signal::One,
    };

    let bit2: FourBit = "1001".parse()?;
    assert_eq!(bit1, bit2);

    Ok(())
}
