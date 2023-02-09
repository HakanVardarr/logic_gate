use logic_gates::bytes::FourBit;
use logic_gates::gates::*;
use logic_gates::Signal;

fn main() {
    let bits1 = "1111".parse::<FourBit>().unwrap();
    let bits2 = "1111".parse::<FourBit>().unwrap();
    let carry = Signal::Zero;

    let (sum, car) = FourBitAdder::send_signal(&carry, &bits1, &bits2);
    display(&bits1, &bits2, &sum, &car)
}

fn display(bits1: &FourBit, bits2: &FourBit, sum: &FourBit, car: &Signal) {
    println!("|----------------------|");
    println!(
        "| Input 1 : {}{}{}{}  = {} |",
        bits1.bit1,
        bits1.bit2,
        bits1.bit3,
        bits1.bit4,
        bits1.convert()
    );
    println!(
        "| Input 2 : {}{}{}{}  = {} |",
        bits2.bit1,
        bits2.bit2,
        bits2.bit3,
        bits2.bit4,
        bits2.convert()
    );
    println!("|----------------------|");
    if car == &Signal::One {
        println!(
            "| Output  : {car}{}{}{}{} = {} |",
            sum.bit1,
            sum.bit2,
            sum.bit3,
            sum.bit4,
            sum.convert() + 16
        );
    } else {
        println!(
            "| Output  : {car}{}{}{}{} = {} |",
            sum.bit1,
            sum.bit2,
            sum.bit3,
            sum.bit4,
            sum.convert()
        );
    }
    println!("|----------------------|");
}