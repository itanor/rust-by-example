mod string;
use string as my_str;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn main() {
    println!("this project has two binaries!");
    println!("one is this main!");
    println!("another is located in src/bin!");
    println!("also have a Makefile!");

    println!("dummy: {}", my_str::dummy_function());
    println!("{}", get_message());
}
