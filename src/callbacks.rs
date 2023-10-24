#![allow(unused)]

#[derive(Debug)]
enum Event {
    Fire(u32)
}

type Callback<'c> = dyn FnMut(&Event) + 'c;

struct Dispatcher<'c> {
    callbacks: Vec<Option<Box<Callback<'c>>>>
}

impl<'c> Dispatcher<'c> {
    fn new() -> Self {
        Self { callbacks: Vec::new() }
    }

    fn register(&mut self, callback: Box<Callback<'c>>) -> usize {
        self.callbacks.push(Some(callback));
        self.callbacks.len() - 1
    }

    fn deregister(&mut self, index: usize) {
        self.callbacks[index] = None;
    }

    fn dispatch(&mut self, event: &Event) {
        for callback in self.callbacks.iter_mut().flatten() {
            (*callback)(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn callback() {
        const N: u32 = 10;
        
        let count = Cell::new(0);

        let mut dispatcher = Dispatcher::new();
        let count_handler = dispatcher.register(Box::new(|event: &Event| {
            match event {
                Event::Fire(n) => count.set(count.get() + n)
            }
        }));

        let dbg_handler = dispatcher.register(Box::new(|event: &Event| {
            match event {
                Event::Fire(_) => dbg!(&count)
            };
        }));

        for _ in 0..N {
            let event = Event::Fire(1);
            dispatcher.dispatch(&event);
        }
        assert_eq!(count.get(), N);

        dispatcher.deregister(dbg_handler);
        for _ in 0..N {
            let event = Event::Fire(2);
            dispatcher.dispatch(&event);
        }
        assert_eq!(count.get(), 3 * N);

        dispatcher.deregister(count_handler);
        dispatcher.dispatch(&Event::Fire(100));

        assert_eq!(count.get(), 3 * N);
    }
}
