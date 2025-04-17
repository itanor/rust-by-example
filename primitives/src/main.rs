fn literals_and_operators() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("1e4 is {} -2.5e-3 is {}", 1e4, -2.5e-3);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is is written as {}", 1_000_000u32);
}

fn main() {
    let logical: bool = true;
    println!("{}", logical);

    let a_float: f64 = 1.0;
    println!("{}", a_float);

    let an_integer   = 5i32;
    println!("{}", an_integer);

    let default_float   = 3.0;
    println!("{}", default_float);

    let default_integer = 7;
    println!("{}", default_integer);

    let mut inferred_type = 12;
    println!("{}", inferred_type);
    inferred_type = 4294967296i64;
    println!("{}", inferred_type);

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", my_array);

    let my_tuple = (5u32, 1u8, true, -5.04f32);
    println!("{:?}", my_tuple);

    literals_and_operators();
}
