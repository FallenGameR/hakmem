fn main() {
    println!();
    println!("turn_off_rightmost_1bit => x & (x - 1)");
    let f = turn_off_rightmost_1bit;
    apply(0b_1010_1000, 0b_0000_0000, f);
    zero_test(0b_0010_0000, f, "pow2");
    zero_test(0b_0000_0000, f, "all zeros");

    println!();
    println!("turn_off_rightaligned_1bit_group => x & (x + 1)");
    let f = turn_off_rightaligned_1bit_group;
    apply(0b_1010_0111, 0b_1010_0000, f);
    zero_test(0b_0011_1111, f, "pow2 - 1");
    zero_test(0b_0000_0000, f, "all zeros");
    zero_test(0b_1111_1111, f, "all ones");

    println!();
    println!("turn_off_rightmost_1bit_group => x & (x + (x & -x))");
    let f = turn_off_rightmost_1bit_group;
    apply(0b_0101_1100, 0b_0000_0000, f);
    println!(" can be used to detect if number is pow2-pow2");

    println!();
    println!("turn_on_rightmost_0bit => x | (x + 1)");
    let f = turn_on_rightmost_0bit;
    apply(0b_1010_0111, 0b_1111_1111, f);

    println!();
    println!("turn_on_rightaligned_0bit_group => x | (x - 1)");
    let f = turn_on_rightaligned_0bit_group;
    apply(0b_1010_1000, 0b_1010_1111, f);

    println!();
    println!("mask0_rightmost_1bit => !x | (x - 1)");
    let f = mask0_rightmost_1bit;
    apply(0b_1010_1000, 0b_1111_1101, f);
    println!("all ones if there is none");
    let x = 0b_0000_0000;
    println!(" {:08b}", x);
    println!(" {:08b}", f(x));

    println!();
    println!("mask0_rightaligned_1bit_group => !x | (x + 1)");
    let f = mask0_rightaligned_1bit_group;
    apply(0b_1010_0111, 0b_0000_0000, f);

    println!();
    println!("mask1_rightmost_0bit => !x & (x + 1)");
    let f = mask1_rightmost_0bit;
    apply(0b_1010_0111, 0b_0000_0010, f);
    println!("all zeros if there is none");
    let x = 0b_1111_1111;
    println!(" {:08b}", x);
    println!(" {:08b}", f(x));

    println!();
    println!("mask1_rightaligned_0bit_group => !x & (x - 1)");
    let f = mask1_rightaligned_0bit_group;
    apply(0b_1010_1000, 0b_1111_1111, f);

    println!();
    println!("mask1_rightmost_1bit => x & -x");
    let f = mask1_rightmost_1bit;
    apply(0b_1010_1000, 0b_0000_1000, f);
    println!("all zeros if there is none");
    let x = 0b_0000_0000;
    println!(" {:08b}", x);
    println!(" {:08b}", f(x));

    println!();
    println!("mask1_extended_rightaligned_0bit_group => x ^ (x - 1)");
    let f = mask1_extended_rightaligned_0bit_group;
    apply(0b_1010_1000, 0b_0000_0001, f);
}

fn apply<F>(start: u8, stop: u8, f: F)
where F: Fn(u8) -> u8 {
    let mut x = start;

    println!(" {:08b}", x);
    while x != stop {
        x = f(x);
        print!(" {:08b}", x);

        if x == stop {
            if f(x) == stop {
                println!(" - stays the same");
            }
            else if f(f(x)) == stop {
                println!(" - alternates the last two");
            }
            else {
                println!(" - UNFINISHED SEQUENCE");
            }
        }
        else{
            println!();
        }
    }
}

fn zero_test<F>(x: u8, f: F, info: &str) where F: Fn(u8) -> u8 {
    if f(x) == 0 {
        println!(" {:08b} -> {}", x, info)
    }
    else{
        println!(" {:08b} -> NOT ZERO, no {}", x, info)
    }
}

/// 0101 1000 => 0101 0000 => x & (x - 1)
/// 0 test to test if number is pow2 or zero
fn turn_off_rightmost_1bit(x: u8) -> u8 {
    x & x.wrapping_sub(1)
}

/// 0101 0111 => 0101 0000 => x & (x + 1)
fn turn_off_rightaligned_1bit_group(x: u8) -> u8 {
    x & x.wrapping_add(1)
}

/// 0101 1100 => 0100 0000 => x & (x + (x & -x))
/// 0000 0100: x & -x
/// 0110 0000: x + [1]
/// 0100 0000: x & [2]
/// 0101 1100 => 0100 0000 => x & (1 + (x | (x-1)))
/// 0101 1111: x | (x-1)
/// 0110 0000: 1 + [1]
/// 0100 0000: x & [2]
fn turn_off_rightmost_1bit_group(x: u8) -> u8 {
    let rightmost_bit = mask1_rightmost_1bit(x);
    x & x.wrapping_add(rightmost_bit)

    // Alternative formula
    // x & (1 + turn_on_rightaligned_0bit_group(x))
}

/// 0101 0111 => 0101 1111 => x | (x + 1)
fn turn_on_rightmost_0bit(x: u8) -> u8 {
    x | x.wrapping_add(1)
}

/// 0101 1000 => 0101 1111 => x | (x - 1)
fn turn_on_rightaligned_0bit_group(x: u8) -> u8 {
    x | x.wrapping_sub(1)
}

/// 0101 1000: x => 1111 0111 => !x | (x-1)
/// 1010 0111: !x
/// 0101 0111: x-1
fn mask0_rightmost_1bit(x: u8) -> u8 {
    !x | x.wrapping_sub(1)
}

/// 0101 0111: x => 0000 1000 => !x & (x+1)
/// 0101 1000: x+1
/// 1010 1000: !x
fn mask1_rightmost_0bit(x: u8) -> u8 {
    !x & x.wrapping_add(1)
}

/// 0101 1000: x => 0000 1000 => x & -x
/// 1010 0111: !x
/// 1010 1000: -x = !x + 1; x + (-x) = 0
fn mask1_rightmost_1bit(x: u8) -> u8 {
    let x = x as i8;
    let result = x & -x;
    result as u8
}

/// 0101 1000 => 0000 0111 => !x & (x-1)
/// 0101 0111: x-1
/// 1010 0111: !x
fn mask1_rightaligned_0bit_group(x: u8) -> u8 {
    !x & x.wrapping_sub(1)

    // Alternative formulas that lack instruction level parrallelizm:
    // 1) !(x | -x)
    // 2) (x & -x) - 1
}

/// 0101 0111: x => 1111 1000 => !x | (x+1)
/// 1010 1000: !x
/// 0101 1000: x+1
fn mask0_rightaligned_1bit_group(x: u8) -> u8 {
    !x | x.wrapping_add(1)
}

/// 0101 1000 => 0000 1111 => x ^ (x - 1)
/// 0101 0111: x-1
fn mask1_extended_rightaligned_0bit_group(x: u8) -> u8 {
    x ^ x.wrapping_sub(1)
}