fn main() {
    fizzbuzz_to(100);
    asociated_functions_and_methods();
    closures();
    capturing();
    as_input_parameters();
    type_anonymity();
    input_functions();
    as_ouput_parameters();
    examples_iterator_any();
    examples_searching_through_iterators();
    high_order_functions();
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for i in 1..=n {
        fizzbuzz(i);
    }
}

#[allow(dead_code)]
fn asociated_functions_and_methods() {
    struct Point {
        x: f64,
        y: f64,
    }
    impl Point {
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    impl Rectangle {
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // Pair owns resources: two heap allocated integers
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        fn destroy(self) {
            let Pair(first, second) = self;
            println!("destroying Pair({}, {})", first, second);
        }
    }

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("rectangle perimeter: {}", rectangle.perimeter());
    println!("rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

fn closures() {
    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    let one = || 1;
    println!("closure returning one: {}", one());
}

fn capturing() {
    use std::mem;

    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;

    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();

    // -------
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // remove 'move' to not cause an error
    // println!("there are {} elements in vec", haystack.len());
}

fn as_input_parameters() {
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    use std::mem;

    let greeting = "hello";

    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    let mut number = "6";

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("i said {}.", greeting);

        let parsed = number.parse::<i32>();
        match parsed {
            Ok(n) => println!("parsed is: {}", n),
            Err(e) => println!("got error: {}", e),
        }
        number = "fdsa";
        println!("new value is: {}", number);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("then i screamed {}.", farewell);
        println!("now i can sleep, zzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}

fn type_anonymity() {
    // `F` must implement `Fn` for a closure which takes no
    // inputs and returns nothing - exactly what is required
    // for `print`.
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let x = 7;
    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);
    apply(print);
}

fn input_functions() {
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    fn function() {
        println!("i am a function!");
    }

    let closure = || println!("i am a closure!");
    call_me(function);
    call_me(closure);
}

fn as_ouput_parameters() {
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("this is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("this is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("this is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

fn examples_iterator_any() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // `iter()` only borrows `vec1` and its elements, so they can be used again
    println!("vec1 len: {}", vec1.len());
    println!("first element of vec1 is: {}", vec1[0]);

    // `into_iter()` does move `vec2` and its elements, so they cannot be used again
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());
    // TODO: uncomment two lines above and see compiler errors.

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
    // can use array2 again because i32 implements Copy...
    println!("array2: {:?}", array2[0]);

    #[derive(Debug)]
    struct MyType {
        number: i32,
    }
    let array3 = [MyType { number: 1 }, MyType { number: 2 }];
    println!("2 in array3: {}", array3.into_iter().any(|t| t.number == 2));
    // cannot use array3 again because MyType doesn't implements Copy...
    // println!("array3: {:?}", array3[0]);
}

fn examples_searching_through_iterators() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [3, 4, 5];

    println!("find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );

    let vec = vec![1, 9, 3, 3, 13, 2];
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}

fn high_order_functions() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    println!("fin the sum of all the numbers with squares under 1000");

    let upper = 1000;
    // imperative approach
    let mut acc = 0;
    // iterate infinity...
    for n in 0.. {
        let n_aquared = n * n;
        if n_aquared >= upper {
            break;
        } else if is_odd(n_aquared) {
            acc += n_aquared;
        }
    }
    println!("imperative style: {}", acc);

    // functional approach
    let sum_of_squared_od_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();
    println!("functional style: {}", sum_of_squared_od_numbers);
}
