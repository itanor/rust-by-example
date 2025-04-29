use std::mem;

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

fn analyze_slice(slice: &[i32]) {
    println!("first element of slice: {}", slice[0]);
    println!("slice has {} elements.", slice.len());
}

fn array_and_slices() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    println!("number of elements ins array: {}", xs.len());
    println!("array occupies {} bytes.", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len()  + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}, {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
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
    array_and_slices();
}
