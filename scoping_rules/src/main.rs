fn crate_box() {
    // allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // _box1 is destroyed here, and memory gets freed
}

fn main() {
    // allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // a nested scope
    {
        // allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // _box3 is destroyed here, and memory gets freed
    }

    // creating lots of boxes
    // there's no need to manually free memory
    for _ in 0u32..1_000 {
        crate_box();
    }

    // _box2 is destroyed here, and memory gets freed

    test_drop();
    ownership_and_moves();
    mutability();
    partial_moves();
    borrowing();
    borrow_mutability();
    aliasing();
    ref_pattern();
    lifetime_explicit_annotation();
    lifetime_functions();
    lifetime_methods();
    lifetime_structs();
    lifetime_traits();
    lifetime_bounds();
    lifetime_coersion();
    static_lifetime();
    lifetime_elision();
}

fn test_drop() {
    struct ToDrop;

    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }

    let _to_drop = ToDrop;
    {
        let _to_drop1 = ToDrop;
    }
    println!("Made a drop!");
}

fn ownership_and_moves() {
    // this function takes ownership of the heap allocated memory
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains: {}", c);
        
        // `c` is destroyed and the memory freed
    }
    // stack allocated integer
    let x = 5u32;

    // *copy* `x` into `y` - no resources are moved
    let y = x;

    // both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is apointer to a heap allocated integer
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // *move* `a` into `b`
    let b = a;
    // the pointer address of `a` is copied (not the data) into `b`.
    // both are now pointers to rhe same heap allocated data, but
    // `b` now owns it.

    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    // println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    // println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
}

fn mutability() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    // mutability error
    // * immutable_box = 4;

    // *move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    // modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

fn partial_moves() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    // Error! cannot move out of a type which implements the `Drop` trait
    //impl Drop for Person {
    //    fn drop(&mut self) {
    //        println!("Dropping the person struct {:?}", self)
    //    }
    //}
    // TODO ^ Try uncommenting these lines

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person {name, ref age} = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    // println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it us not moved
    println!("The person's age from person struct is {}", person.age);
}

fn borrowing() {
    // this function takes ownership of a box and destroys it
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    // this function borrows an i32
    fn borrow_i32(borrowed_i32: &i32) {
        println!("this int is {}", borrowed_i32);
    }

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // borrow the contents of the box. ownership is not taken,
    // so the contents can be borrowed again
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // error!
        // can't destroy `boxed_i32` while the inner value is borrowed later in scope
        // eat_box_i32(boxed_i32);

        // attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
    }

    // `boxed_i32` can now give up ownership to `eat_box_i32` and be destroyed
    eat_box_i32(boxed_i32);
}

fn borrow_mutability() {
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    // this function takes a reference toa book
    fn borrow_book(book: &Book) {
        println!("I immutably borrowed {} - {} edition", book.title, book.year);
    }
    
    // this function takes a reference to a mutable book and changes `year`
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }

    let immutabook = Book{
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutabook);

    new_edition(&mut mutabook);
}

fn aliasing() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut point = Point{
        x: 0,
        y: 0,
        z: 0,
    };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Data can be accessed via the references and the original owner
    println!("Point has coordinates: ({}, {}, {})", 
        borrowed_point.x, another_borrow.y, point.z);

    // Error! Can't borrow `point` as mutable because it's currently
    // borrowed as immutable.
    // let mutable_borrow = &mut point;
    // TODO ^ Try uncommenting this line

    // the borrowed values are used again here
    println!("Point has coordinates: ({}, {}, {})", 
        borrowed_point.x, another_borrow.y, point.z);

    // the immutable references are no longer used for the rest of the code so
    // it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! Can't borrow `point` as immutable because it's currently
    // borrowed as mutable.
    // let y = &point.y;
    // TODO ^ Try uncommenting this line

    // Error! Can't print because `println!` takes an immutable reference.
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ Try uncommenting this line

    // Ok! mutable references can be passed as immutable to `println`
    println!("Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // the immutable reference is no longer used for the rest of the code so it
    // is possible to reborrow
    let new_borrowed_point = &point;
    println!("Point has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);

}

fn ref_pattern() {
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point {
        x: 0,
        y: 0,
    };

    // `ref` is also valid when destructuring a struct.
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        let Point {x: ref ref_to_x, y: _} = point;

        // Return a copy of the `x` field of `point`.
        *ref_to_x
    };

    // a mutable copy of `point`
    let mut mutable_point = point;

    {
        // `ref` can be paired with `mut` to take mutable references.
        let Point {x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Mutate the `y` field of `mutable_point` via a mutable reference.
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // Destructure `mutable_tuple` to change the value of `last`.
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}

fn lifetime_explicit_annotation() {
    // `print_refs` takes two references to `i32` which have different
    // lifetimes `'a` and `'b`. These two lifetimes must both be at
    // least as long as the function `print_refs`.
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    // A function which takes no arguments, but has a lifetime parameter `'a`.
    fn failed_borrow<'a>() {
        let _x = 12;

        // ERROR: `_x` does not live long enough
        // let _y: &'a i32 = &_x;

        // Attempting to use the lifetime `'a` as an explicit type annotation 
        // inside the function will fail because the lifetime of `&_x` is shorter
        // than that of `_y`. A short lifetime cannot be coerced into a longer one.
    }

    // Create variables to be borrowed below.
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower. 
    // In other words, the lifetime of `four` and `nine` must 
    // be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be 
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}

fn lifetime_functions() {
    // One input reference with lifetime `'a` which must live
    // at least as long as the function.
    fn print_one<'a>(x: &'a i32) {
        println!("`print_one`: x is {}", x);
    }

    // Mutable references are possible with lifetimes as well.
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    // Multiple elements with different lifetimes. In this case, it
    // would be fine for both to have the same lifetime `'a`, but
    // in more complex cases, different lifetimes may be required.
    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("`print_multi`: x is {}, y is {}", x, y);
    }

    // Returning references that have been passed in is acceptable.
    // However, the correct lifetime must be returned.
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    //fn invalid_output<'a>() -> &'a String { &String::from("foo") }
    // The above is invalid: `'a` must live longer than the function.
    // Here, `&String::from("foo")` would create a `String`, followed by a
    // reference. Then the data is dropped upon exiting the scope, leaving
    // a reference to invalid data to be returned.

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

fn lifetime_methods() {
    struct Owner(i32);

    impl Owner {
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }

        fn print<'a>(&'a self) {
            println!("`print`: {}", self.0);
        }
    }

    let mut owner = Owner(18);
    owner.add_one();
    owner.print();
}

fn lifetime_structs() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Borrowed<'a>(&'a i32);

    #[derive(Debug)]
    #[allow(dead_code)]
    struct NameBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NameBorrowed{x: &x, y: &y};
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);

    let b = Borrowed(&x);
    println!("x is borrowed in {:?}", b);
}

fn lifetime_traits() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Borrowed<'a> {
        x: &'a i32,
    }

    impl<'a> Default for Borrowed<'a> {
        fn default() -> Self {
            Self { x: &10 }
        }
    }

    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}

fn lifetime_bounds() {
    use std::fmt::Debug; // Trait to bound with.

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Ref<'a, T: 'a>(&'a T);
    // `Ref` contains a reference to a generic type `T` that has
    // some lifetime `'a` unknown by `Ref`. `T` is bounded such that any
    // *references* in `T` must outlive `'a`. Additionally, the lifetime
    // of `Ref` may not exceed `'a`.

    // A generic function which prints using the `Debug` trait.
    fn print<T>(t: T) where 
        T: Debug {
        println!("`print`: t is {:?}", t);
    }

    // Here a reference to `T` is taken where `T` implements
    // `Debug` and all *references* in `T` outlive `'a`. In
    // addition, `'a` must outlive the function.
    fn print_ref<'a, T>(t: &'a T) where 
        T: Debug + 'a {
        println!("`print_ref`: t is {:?}", t);
    }

    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

fn lifetime_coersion() {
    // Here, Rust infers a lifetime that is as short as possible.
    // The two references are then coerced to that lifetime.
    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    // `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
    // Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }

    let first = 2;  // longer lifetime
    {
        let second = 3; // shorter lifetime
        println!("the product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}

fn static_lifetime() {
    // Make a constant with `'static` lifetime.
    static NUM: i32 = 18;

    // Returns a reference to `NUM` where its `'static`
    // lifetime is coerced to that of the input argument.
    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    {
        // Make a `string` literal and print it:
        let static_string = "i'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;
        // Coerce `NUM` to lifetime of `lifetime_num`:
        let corced_static = coerce_static(&lifetime_num);
        println!("coerce_static: {}", corced_static);
    }

    println!("NUM: {} stays accessible!", NUM);

    use std::fmt::Debug;

    fn print_it(input: impl Debug + 'static) {
        println!("'static value passed in is: {:?}", input);
    }

    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    // print_it(&i);
}

fn lifetime_elision() {

    // `elided_input` and `annotated_input` essentially have identical signatures
    // because the lifetime of `elided_input` is inferred by the compiler:
    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x);
    }

    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x);
    }

    // Similarly, `elided_pass` and `annotated_pass` have identical signatures
    // because the lifetime is added implicitly to `elided_pass`:
    fn elided_pass(x: &i32) -> &i32 { x }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x))
}