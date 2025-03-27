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
}
