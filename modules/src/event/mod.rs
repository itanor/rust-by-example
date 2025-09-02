#[derive(Debug)]
pub struct Event {
    id: i32,
}

impl Event {
    pub fn new() -> Event {
        Event { id: 0 }
    }

    pub fn inc(self) -> Event {
        Event { id: self.id + 1 }
    }
}
