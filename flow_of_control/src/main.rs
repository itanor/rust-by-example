use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    if_else();
    _loop();
    nesting_and_labels();
    returning_from_loops();
    _while();
    ranges();
    for_and_iterators();
    _match();
    destructuring();
    arrays_and_slices();
    enums();
    pointers_and_ref();
    structs();
    guards();
    binding();
    if_let();
    let_else();
    while_let();
}

fn if_else() {
    let n = 5;
    
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
    
    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase then-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };
    
    println!("{} -> {}", n, big_n);
}

fn _loop() {
    let mut count = 0u32;

    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("ok, that's enough");
            break;
        }
    }
}

#[allow(unused_labels, unreachable_code)]
fn nesting_and_labels() {
    'outer: loop {
        println!("entered the outer loop");

        'inner: loop {
            println!("entered the inner loop");

            // this would break only the inner loop
            // break;
            
            break 'outer;
        }
        println!("this point will never be reached");
    }
    println!("exited the outer loop");
}

fn returning_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}

fn _while() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

fn ranges() {
    for n in 1..101 /*101 is exclusive*/ {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    for n in 1..=100 /*100 is inclusive*/ {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_and_iterators() {
    {
        let names = vec!["bob", "frank", "ferris"];
        for name in names.iter() {
            match name {
                &"ferris" => println!("there is a rustacean among us!"),
                _ => println!("hello {}", name),
            }
        }
        println!("names: {:?}", names);
    }
    {
        let names = vec!["bob", "frank", "ferris"];
        for name in names.into_iter() {
            match name {
                "ferris" => println!("there is a rustacean among us!"),
                _ => println!("hello {}", name),
            }
        }
        // already consumed by into_iter()...
        // println!("names: {:?}", names);
    }
    {
        let mut names = vec!["bob", "frank", "ferris"];
        for name in names.iter_mut() {
            match name {
                &mut "ferris" => println!("there is a rustacean among us!"),
                _ => println!("hello {}", name),
            }
        }
        println!("names: {:?}", names);
    }
}

fn _match() {
    let number = 13;

    println!("tell me about {}", number);
    match number {
        1 =>                  println!("one!"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        13..=19 =>            println!("a teen"),
        _ =>                  println!("ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("{} -> {}", boolean, binary);
}

fn destructuring() {
    // tuples
    let triple = (0, -2, 3);
    println!("tell me about {:?}", triple);

    // match can be used to destructure a tuple
    match triple {
        (0, y, z) => println!("first is: `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("first is `1` and the rest doesn't matter"),
        (.., 2) => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("first is `3`, last is `4`, and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    let max = triple.max((1, -2, 4));
    println!("max is {:?}", max);

    let mut hasher = DefaultHasher::new();
    max.hash(&mut hasher);
    println!("hash of max: {:x}", hasher.finish());
}

fn arrays_and_slices() {
    let array = [1, -2, 6];
    match array {
        [0, second, third] => println!(
            "array[0] = 0, array[1] = {}, array[2] = {}", 
            second, third
        ),
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        [3, second,tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        [first, middle@.., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}

fn enums() {
    #[allow(dead_code)]
    enum Color {
        Red, 
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    println!("what color is it?");
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red => println!("the color is Red!"),
        Color::Blue => println!("the color is Blue!"),
        Color::Green => println!("the color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
    }
}

fn pointers_and_ref() {
    let reference = &4;
    match reference {
        &val => println!("got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("git a reference to a value: {:?}", r),
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("we added 10. `mut_value`: {:?}", m);
        },
    }
}

fn structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo {x: (1, 2), y: 3};
    match foo {
        Foo { x: (1, b), y} => 
            println!("First of x is 1, b = {}, y = {} ", b, y),
        Foo { y: 2, x: i} => 
            println!("y is 2, i = {:?}", i),
        Foo { y, .. } =>
            println!("y = {}, we don't care about x", y),
    }

    let faa = Foo{x: (1, 2), y: 3};
    let Foo {x: x0, y: y0} = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    struct Bar {
        foo: Foo,
    }

    let bar = Bar{foo: faa};
    let Bar {foo: Foo { x: nested_x, y: nested_y }} = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}

#[allow(dead_code)]
fn guards() {
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }

    let temperature = Temperature::Celsius(35);
    match temperature {
        Temperature::Celsius(t) if t > 30 => 
            println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => 
            println!("{}C is equal to or below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => 
            println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => 
            println!("{}F is equal to or below 86 Fahrenheit", t),
    }
}

fn binding() {
    fn age() -> u32 {
        15
    }

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1 ..=12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}

fn if_let() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // enums
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred: {}", value);
    }
}

fn let_else() {
    use std::str::FromStr;

    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };
        let Ok(count) = u64::from_str(count_str) else {
            panic!("Can't parse integer: '{count_str}'");
        };
        (count, item)
    }

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}

fn while_let() {
    let mut optional = Some(0);
    // with 'loop'
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            _ => {
                break;
            },
        }
    }

    // with 'while let'
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}