fn main() {
    println!(
        "i8({}ケタ):     {} 〜 {}",
        i8::MAX.to_string().len(),
        i8::MIN,
        i8::MAX
    );
    println!(
        "u8({}ケタ):     {} 〜 {}",
        u8::MAX.to_string().len(),
        u8::MIN,
        u8::MAX
    );
    println!(
        "i16({}ケタ):    {} 〜 {}",
        i16::MAX.to_string().len(),
        i16::MIN,
        i16::MAX
    );
    println!(
        "u16({}ケタ):    {} 〜 {}",
        u16::MAX.to_string().len(),
        u16::MIN,
        u16::MAX
    );
    println!(
        "i32({}ケタ):    {} 〜 {}",
        i32::MAX.to_string().len(),
        i32::MIN,
        i32::MAX
    );
    println!(
        "u32({}ケタ):    {} 〜 {}",
        u32::MAX.to_string().len(),
        u32::MIN,
        u32::MAX
    );
    println!(
        "i64({}ケタ):    {} 〜 {}",
        i64::MAX.to_string().len(),
        i64::MIN,
        i64::MAX
    );
    println!(
        "u64({}ケタ):    {} 〜 {}",
        u64::MAX.to_string().len(),
        u64::MIN,
        u64::MAX
    );
    println!(
        "i128({}ケタ):   {} 〜 {}",
        i128::MAX.to_string().len(),
        i128::MIN,
        i128::MAX
    );
    println!(
        "u128({}ケタ):   {} 〜 {}",
        i128::MAX.to_string().len(),
        u128::MIN,
        u128::MAX
    );
    println!(
        "isize({}ケタ):  {} 〜 {}",
        isize::MAX.to_string().len(),
        isize::MIN,
        isize::MAX
    );
    println!(
        "usize({}ケタ):  {} 〜 {}",
        usize::MAX.to_string().len(),
        usize::MIN,
        usize::MAX
    );
}
