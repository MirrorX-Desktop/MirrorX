use std::sync::atomic::{AtomicU16, Ordering};

pub struct CallIdGenerator {
    call_id: AtomicU16,
}

impl CallIdGenerator {
    pub fn new() -> Self {
        Self {
            call_id: AtomicU16::new(1),
        }
    }

    pub fn next(&self) -> u16 {
        loop {
            let id = self.call_id.fetch_add(1, Ordering::AcqRel);
            if id == 0 {
                continue;
            }
            return id;
        }
    }
}
