fn main() {
    if_else();
    _loop();
    nesting_and_labels();
    returning_from_loops();
    _while();
    ranges();
    for_and_iterators();
    _match();
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