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

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn tuples() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64, 'a', true);

    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple first value: {}", long_tuple.1);

    let tuple_of_tuples = (
        (1u8, 2u16, 2u32),
        (4u64, -1i8, -2i16),
    );
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("Pair: {:?}", pair);
    println!("The reverse pair is {:?}", reverse(pair));

    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix: {:?}", matrix);
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
    tuples();
}
