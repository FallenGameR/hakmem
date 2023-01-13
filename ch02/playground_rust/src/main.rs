fn main() {

    println!();
    let f = turn_off_rightmost_1bit;
    let x = 0b_1010_1000;
    println!("turn_off_rightmost_1bit");
    println!("{:08b}\n{:08b}", x, f(x));
    let x = 0b_0010_0000;
    println!("{:08b} is pow2 or zero -> {}", x, f(x) == 0);
    let x = 0b_0000_0000;
    println!("{:08b} is pow2 or zero -> {}", x, f(x) == 0);
    let x = 0b_0000_0101;
    println!("{:08b} is pow2 or zero -> {}", x, f(x) == 0);

    println!();
    let f = turn_on_rightmost_0bit;
    let x = 0b_1010_0111;
    println!("turn_on_rightmost_0bit");
    println!("{:08b}\n{:08b}", x, f(x));
    let x = 0b_1111_1111;
    println!("{:08b}\n{:08b}", x, f(x));


}

/// 0101 1000 => 0101 0000
/// 0 test to test if number is pow2 or zero
fn turn_off_rightmost_1bit(x: u8) -> u8 {
    // x & (x - 1)
       x & x.wrapping_sub(1)
}

/// 0101 0111 => 0101 1111
fn turn_on_rightmost_0bit(x: u8) -> u8 {
    // x | (x + 1)
       x | x.wrapping_add(1)
}

