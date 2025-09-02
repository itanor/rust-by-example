mod emitter;
mod event;

use emitter::{call_indirectly, emitter_func};
use event::Event;

fn main() {
    let event = Event::new();
    emitter_func(&event);
    let inc_event = event.inc();
    emitter_func(&inc_event);
    call_indirectly();
}
