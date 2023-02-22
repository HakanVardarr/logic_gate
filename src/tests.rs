use crate::bytes::FourBit;
use crate::gates::*;
use crate::Signal;
use std::error::Error;

#[test]
fn fourbit_parse() -> Result<(), Box<dyn Error>> {
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
#[test]
fn mux_gate() -> Result<(), Box<dyn Error>> {
    let result1 = Mux::send_signal(&Signal::One, &Signal::One, &Signal::One);
    let result2 = Mux::send_signal(&Signal::One, &Signal::One, &Signal::Zero);
    let result3 = Mux::send_signal(&Signal::One, &Signal::Zero, &Signal::One);
    let result4 = Mux::send_signal(&Signal::One, &Signal::Zero, &Signal::Zero);
    let result5 = Mux::send_signal(&Signal::Zero, &Signal::One, &Signal::One);
    let result6 = Mux::send_signal(&Signal::Zero, &Signal::One, &Signal::Zero);
    let result7 = Mux::send_signal(&Signal::Zero, &Signal::Zero, &Signal::One);
    let result8 = Mux::send_signal(&Signal::Zero, &Signal::Zero, &Signal::Zero);

    assert_eq!(result1, Signal::One);
    assert_eq!(result2, Signal::Zero);
    assert_eq!(result3, Signal::One);
    assert_eq!(result4, Signal::Zero);
    assert_eq!(result5, Signal::One);
    assert_eq!(result6, Signal::One);
    assert_eq!(result7, Signal::Zero);
    assert_eq!(result8, Signal::Zero);

    Ok(())
}
#[test]
fn xor_gate() -> Result<(), Box<dyn Error>> {
    let result1 = Xor::send_signal(&Signal::One, &Signal::One);
    let result2 = Xor::send_signal(&Signal::One, &Signal::Zero);
    let result3 = Xor::send_signal(&Signal::Zero, &Signal::One);
    let result4 = Xor::send_signal(&Signal::Zero, &Signal::Zero);

    assert_eq!(result1, Signal::Zero);
    assert_eq!(result2, Signal::One);
    assert_eq!(result3, Signal::One);
    assert_eq!(result4, Signal::Zero);

    Ok(())
}
#[test]
fn demux_gate() -> Result<(), Box<dyn Error>> {
    let (o1r1, o2r1) = Demux::send_signal(&Signal::One, &Signal::One);
    let (o1r2, o2r2) = Demux::send_signal(&Signal::One, &Signal::Zero);
    let (o1r3, o2r3) = Demux::send_signal(&Signal::Zero, &Signal::One);
    let (o1r4, o2r4) = Demux::send_signal(&Signal::Zero, &Signal::Zero);

    assert_eq!(o1r1, Signal::Zero);
    assert_eq!(o2r1, Signal::One);
    assert_eq!(o1r2, Signal::One);
    assert_eq!(o2r2, Signal::Zero);
    assert_eq!(o1r3, Signal::Zero);
    assert_eq!(o2r3, Signal::Zero);
    assert_eq!(o1r4, Signal::Zero);
    assert_eq!(o2r4, Signal::Zero);

    Ok(())
}
#[test]
fn negative_test() -> Result<(), Box<dyn Error>> {
    let n1: FourBit = "1001".parse()?;
    assert_eq!(n1.convert_int(), -7);

    Ok(())
}

#[test]
fn reverse_bits() -> Result<(), Box<dyn Error>> {
    let n1: FourBit = "1001".parse()?;
    let n2: FourBit = "0110".parse()?;

    assert_eq!(n1.reverse(), n2);

    Ok(())
}
#[test]
fn and3_test() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        Signal::Zero,
        And3Way::send_signal(&Signal::Zero, &Signal::One, &Signal::One)
    );

    Ok(())
}

#[test]
fn mux4_test() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        Signal::One,
        Mux4Way::send_signal(
            &Signal::Zero,
            &Signal::Zero,
            &Signal::One,
            &Signal::Zero,
            &Signal::Zero,
            &Signal::One
        )
    );

    Ok(())
}
#[test]
fn alu_test() -> Result<(), Box<dyn Error>> {
    let a = Alu::send_signal(
        &Signal::Zero,
        &Signal::Zero,
        &Signal::Zero,
        &Signal::Zero,
        &Signal::One,
    );
    assert_eq!(a.0, Signal::One);
    assert_eq!(a.1, Signal::Zero);

    Ok(())
}
#[test]
fn substract_test() -> Result<(), Box<dyn Error>> {
    let a: FourBit = "0111".parse()?;
    let b: FourBit = "0110".parse()?;

    let (sum, _) = FourBitAdder::send_signal(&Signal::Zero, &Signal::One, &a, &b);

    assert_eq!("0001".parse::<FourBit>().unwrap(), sum);

    Ok(())
}
#[test]
fn alu4way_test() -> Result<(), Box<dyn Error>> {
    let a: FourBit = "0111".parse()?;
    let b: FourBit = "0110".parse()?;

    let sum = Alu4Way::send_signal(&a, &b, &Signal::One, &Signal::Zero, &Signal::Zero);
    assert_eq!("0110".parse::<FourBit>().unwrap(), sum);

    Ok(())
}
