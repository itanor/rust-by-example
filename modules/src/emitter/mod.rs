mod emit;
use crate::event::Event;
use emit::emit_function;

pub fn emitter_func(e: &Event) {
    println!("emitter_func");
    println!("event is: {:?}", e);
    emit_function();
}

mod k_emitter;
use k_emitter::k_emitter_function;

pub fn call_indirectly() {
    k_emitter_function();
}
