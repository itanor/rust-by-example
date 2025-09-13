use std::fmt::{Debug, Display};
use std::marker::PhantomData;

fn main() {
    simple_generics();
    functions();
    implementation();
    traits();
    bounds();
    empty_bounds();
    multiple_bounds();
    where_clauses();
    new_type_idiom();
    associated_items();
    associated_types();
    phantom_type_parameters();
    test_case_unit_clarification();
}

fn simple_generics() {
    struct A;
    struct Single(A);
    struct SingleGen<T>(T);

    let _s = Single(A);
    let _char = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
}

fn functions() {
    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}

fn implementation() {
    struct Val {
        val: f64,
    }

    struct GenVal<T> {
        gen_val: T,
    }

    // simple impl of Val
    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    // impl of GenVal for a generic type `T`
    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{} {}", x.value(), y.value());

    let c = GenVal { gen_val: 'A' };
    println!("{}", c.value());
}

fn traits() {
    struct Empty;
    struct Null;

    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an
        // additional single parameter `T` and does nothing with it.
        fn double_drop(self, _: T);
    }

    // Implement `DoubleDrop<T>` for any generic parameter `T` and caller `U`.
    impl<T, U> DoubleDrop<T> for U {
        // This method takes ownership of both passed arguments,
        // deallocating both.
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}

fn bounds() {
    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }

    #[allow(dead_code)]
    struct Triangle {
        length: f64,
        height: f64,
    }

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length + self.height
        }
    }

    // The generic `T` must implement `Debug`. Regardless
    // of the type, this will work properly.
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    // `T` must implement `HasArea`. Any type which meets
    // the bound can access `HasArea`'s function `area`.
    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    fn consume<T: HasArea + Debug>(t: T) {
        t.area();
    }

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    consume(rectangle);
    // println!("{}", rectangle.height);
    // println!("{}", rectangle.length);
}

fn empty_bounds() {
    struct Cardinal;
    struct Bluejay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red for Cardinal {}
    impl Blue for Bluejay {}

    fn red<T: Red>(_: &T) -> &'static str {
        "red"
    }

    fn blue<T: Blue>(_: &T) -> &'static str {
        "blue"
    }

    let cardinal = Cardinal;
    let blue_jay = Bluejay;
    let _turkey = Turkey;
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    #[allow(dead_code)]
    enum BookFormat {
        Paperback,
        Hardback,
        Ebook,
    }

    #[allow(dead_code)]
    struct Book {
        isbn: i32,
        format: BookFormat,
    }

    impl Book {
        fn new(isbn: i32, format: BookFormat) -> Book {
            Book { isbn, format }
        }
    }
    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }

    let hardback = Book::new(123, BookFormat::Hardback);
    let ebook = Book::new(123, BookFormat::Ebook);

    println!("books are equal: {}", hardback == ebook);
}

fn multiple_bounds() {
    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);
}

fn where_clauses() {
    trait PrintInOption {
        fn print_in_option(self);
    }

    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

// The newtype idiom gives compile time guarantees that the
// right type of value is supplied to a program.
// For example, an age verification function that checks age in years,
// must be given a value of type Years.
fn new_type_idiom() {
    #[allow(dead_code)]
    struct Years(i64);

    #[allow(dead_code)]
    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn is_adult(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(25);
    let age_days = age.to_days();
    println!("Is an adult: {}", is_adult(&age));
    println!("Is an adult: {}", is_adult(&age_days.to_years()));

    let Years(age_as_primitive) = age;
    println!("{}", age_as_primitive);
}

fn associated_items() {
    struct Container(i32, i32);

    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`
        fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`
        fn last(&self) -> i32; // Doesn't explicitly require `A` or `B`
    }

    impl Contains<i32, i32> for Container {
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Contains<A, B>,
    {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

fn associated_types() {
    struct Container(i32, i32);

    trait Contains {
        type A;
        type B;

        fn contains(&self, number_1: &i32, number_2: &i32) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

fn phantom_type_parameters() {
    #[derive(PartialEq)]
    struct PhantomTuple<A, B>(A, PhantomData<B>);

    #[derive(PartialEq)]
    struct PhantomStruct<A, B> {
        first: A,
        phantom: PhantomData<B>
    }

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // Compile-time Error! Type mismatch so these cannot be compared:
    // println!("_tuple1 == _tuple2 yields: {}",
    //           _tuple1 == _tuple2);

    // Compile-time Error! Type mismatch so these cannot be compared:
    // println!("_struct1 == _struct2 yields: {}",
    //           _struct1 == _struct2);
}

fn test_case_unit_clarification() {
    use std::ops::Add;
    use std::marker::PhantomData;

    #[derive(Debug, Clone, Copy)]
    enum Inch {}

    #[derive(Debug, Clone, Copy)]
    enum Mm {}

    /// `f64` already implements the `Clone` and `Copy` traits.
    #[derive(Debug, Clone, Copy)]
    struct Length<Unit>(f64, PhantomData<Unit>);

    /// The `Add` trait defines the behavior of the `+` operator.
    impl<Unit> Add for Length<Unit> {
        type Output = Length<Unit>;

        fn add(self, rhs: Length<Unit>) -> Length<Unit> {
            // `+` calls the `Add` implementation for `f64`
            Length(self.0 + rhs.0, PhantomData)
        }
    }

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm>  = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one foot = {:?} in", two_feet.0);
    println!("one meter + one meter = {:?} mm", two_meters.0);
}

