fn main() {

    println!();
    println!("turn_off_rightmost_1bit => x & (x - 1)");
    let f = turn_off_rightmost_1bit;
    apply(0b_1010_1000, 0b_0000_0000, f);
    println!("pow2 | all zeros");
    zero_test(0b_0010_0000, f);
    zero_test(0b_0000_0000, f);
    zero_test(0b_0000_0101, f);

    println!();
    println!("turn_on_rightmost_0bit => x | (x + 1)");
    let f = turn_on_rightmost_0bit;
    apply(0b_1010_0111, 0b_1111_1111, f);

    println!();
    println!("turn_off_rightaligned_1bit_group => x & (x + 1)");
    let f = turn_off_rightaligned_1bit_group;
    let x = apply(0b_1010_0111, 0b_1010_0000, f);
    println!(" {:08b} same", f(x));
    println!("pow2-1 | all zeros | all ones");
    zero_test(0b_0011_1111, f);
    zero_test(0b_0000_0000, f);
    zero_test(0b_1111_1111, f);
    zero_test(0b_1010_0111, f);

    println!();
    println!("turn_on_rightaligned_0bit_group => x | (x - 1)");
    let f = turn_on_rightaligned_0bit_group;
    let x = apply(0b_1010_1000, 0b_1010_1111, f);
    println!(" {:08b} same", f(x));

    println!();
    println!("mask0_rightmost_1bit => !x | (x-1)");
    let f = mask0_rightmost_1bit;
    apply(0b_1010_1000, 0b_1111_1110, f);
    println!("all ones if there is none");
    let x = 0b_0000_0000;
    println!(" {:08b}", x);
    println!(" {:08b}", f(x));

    println!();
    println!("mask1_rightmost_0bit => !x & (x+1)");
    let f = mask1_rightmost_0bit;
    apply(0b_1010_0111, 0b_0000_0001, f);
    println!("all zeros if there is none");
    let x = 0b_1111_1111;
    println!(" {:08b}", x);
    println!(" {:08b}", f(x));
}

fn apply<F>(start: u8, stop: u8, f: F) -> u8
where F: Fn(u8) -> u8 {
    let mut x = start;
    println!(" {:08b}", x);
    while x != stop {
        x = f(x);
        println!(" {:08b}", x);
    }
    x
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

/// 0101 0111 => 0101 0000
fn turn_off_rightaligned_1bit_group(x: u8) -> u8 {
    // x & (x + 1)
       x & x.wrapping_add(1)
}

/// 0101 1000 => 0101 1111
fn turn_on_rightaligned_0bit_group(x: u8) -> u8 {
    // x | (x - 1)
    x | x.wrapping_sub(1)
}

/// 0101 1000: x => 1111 0111
/// 1010 0111: !x
/// 0101 0111: x-1
fn mask0_rightmost_1bit(x: u8) -> u8 {
    // !x | (x-1)
    !x | x.wrapping_sub(1)
}

/// 0101 0111: x => 0000 1000
/// 0101 1000: x+1
/// 1010 1000: !x
fn mask1_rightmost_0bit(x: u8) -> u8 {
    // !x & (x+1)
    !x & x.wrapping_add(1)
}

fn zero_test<F>(x: u8, f: F) where F: Fn(u8) -> u8 {
    println!(" {:08b} -> {}", x, f(x) == 0)
}
