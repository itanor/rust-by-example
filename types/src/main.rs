#![allow(overflowing_literals)]
fn main() {
    casting();
    literals();
    inference();
    aliasing();
}

fn casting() {
    let decimal = 65.4321_f32;

    // error. no implicit conversion
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("-1 as a u8 is: {}", (-1i8) as u8);
    println!("1000 mod 256 is: {}", 1000 % 256);
    println!("128 as a i16 is: {}", 128 as i16);
    println!("128 as a i8 is: {}", 128 as i8);
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("232 as a i8 is: {}", 232 as i8);
    println!("300.0 as u8 is: {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is: {}", -100.0_f32 as u8);
    println!("nan as u8 is: {}", f32::NAN as u8);

    unsafe {
        println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

fn literals() {
    // suffuxed literals
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed literals
    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn inference() {
    // because of the annotation, the compiler knows that `elem` has type u8
    let elem = 5u8;

    let mut vec = Vec::new();
    // at this point the compler doesn't know exact the type of vec

    vec.push(elem);
    // now the compiler knoes that `vec` is a vector of `u8`s

    println!("{:?}", vec);
}

fn aliasing() {
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    println!("{} nanoseconds + {} inches = {} unit?", 
        nanoseconds, 
        inches, 
        nanoseconds + inches);
}