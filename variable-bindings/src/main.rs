
fn main() {
    variable_bindings();
    mutability();
    scope();
    shadowing();
    declare_first();
    freezing();
}

fn variable_bindings() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();
    
    let copied_integer = an_integer;
    
    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
    
    let _unused_variable = 3u32;
    
    let _noisy_unused_variable = 2u32;
}

fn mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("after mutation: {}", mutable_binding);
}

fn scope() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    // Error: 'short_lived_binding' does not live in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("long live binding: {}", long_lived_binding);
}

fn shadowing() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";
        println!("shadowed inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);
    let shadowed_binding = "def";
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn declare_first() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;

    // Error: give a explicit type...
    // println!("another binding: {}", another_binding);
    another_binding = 1;
    println!("another binding: {}", another_binding);
}

fn freezing() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;
        // error `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
    }
    _mutable_integer = 3;
}