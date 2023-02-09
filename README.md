# Logic Gates

Trying to simulate logic gates in rust programming language.

## Todo:

- 4 bit adder (completed)
- Change the project to a rust lib (completed)
- Negative number support (uncompleted)
- 8 bit (uncompleted)
- 8 bit adder (uncompleted)

## Usage:

You need to import theese if you want to use the features.

```rust
use logic_gates::Signal;
use logic_gates::gates::*;
use logic_gates::bytes::FourByte;
```

## Examples:

```rust
// Importing essentials... (You can check them in usage part)

fn main() -> Result<(), Box<dyn std::error::Error> {
  let byte1: FourBit = "0100".parse()?;
  let byte2: FourBit = "0010".parse()?;
  let carry = Signal::Zero;


  let (sum, car) = FourBitAdder::send_signal(&carry, &bits1, &bits2);


  Ok(())
}
```


In this example you can add two four bytes together. You need to send carry signal as `Signal::Zero` carry signal is usefull if you want to implement 8bit adder.

If you want to display the added four bit you can implement a display function like this.

```rust
fn display(bits1: &FourBit, bits2: &FourBit, sum: &FourBit, car: &Signal) {
    println!(
        "Input 1 :  {}{}{}{}  = {}",
        bits1.bit1,
        bits1.bit2,
        bits1.bit3,
        bits1.bit4,
        bits1.convert()
    );
    println!(
        "Input 2 :  {}{}{}{}  = {}",
        bits2.bit1,
        bits2.bit2,
        bits2.bit3,
        bits2.bit4,
        bits2.convert()
    );

    if car == &Signal::One {
        println!(
            "Output  : {car}{}{}{}{} = {}",
            sum.bit1,
            sum.bit2,
            sum.bit3,
            sum.bit4,
            sum.convert() + 16
        );
    } else {
        println!(
            "Output  : {car}{}{}{}{} = {}",
            sum.bit1,
            sum.bit2,
            sum.bit3,
            sum.bit4,
            sum.convert()
        );
    }
}
```

If you use this function to display. You will get a result like this.

```rust

Input 1 :  0100  = 4
Input 2 :  0010  = 2
Output  : 00110  = 6

```

Extra fifth bit is carry bit it is not a necessary bit do display we are working with 4 bits not with 5 bits. But carry bit is usefull in the future when I develop 8BitAdder.
