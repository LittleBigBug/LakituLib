use std::sync::{RwLock, RwLockWriteGuard};

pub type LakituEventHandlerID<T> = dyn LakituEventHandler<T> + LakituEventID;

pub trait LakituEvent {
    type Data;
    type EventHandler = LakituEventHandlerID<Self::Data>;

    fn fire(&self, data: Self::Data);

    fn add_handler(&self, handler: Self::EventHandler);
    fn destroy_handler(&self, id: usize) -> Result<(), Err>;

    fn get_handler(&self, id: usize) -> Option<Box<Self::EventHandler>>;
    fn get_handlers(&self) -> &Vec<Box<Self::EventHandler>>;
}

pub trait LakituEventHandler<T> {
    type Data = T;

    fn new();
    fn on_fired(&self, data: Box<Self::Data>);
}

pub trait LakituEventID {
    fn set_id(&self, id: usize);
    fn get_id(&self) -> usize;
}

pub trait LakituEventManager {
    fn get_events(&self) -> &RwLock<Vec<dyn LakituEvent>>;
    fn register_event(&self, event: Box<dyn LakituEvent>) -> usize;
    fn register_event_handler(&self, index: usize, event_handler: Box<LakituEventHandlerID<T>>);
}

#[derive(Copy, Clone)]
struct EventCounter {
    cur: usize,
}

impl EventCounter {
    fn next(self) -> EventCounter {
        EventCounter { cur: self.cur + 1 }
    }

    fn start() -> EventCounter {
        EventCounter { cur: 0 }
    }
}

impl From<EventCounter> for usize {
    fn from(counter: EventCounter) -> usize {
        counter.cur
    }
}

#[macro_export]
macro_rules! define_lakitu_event {
    ($event:ident, $data:ty) => {
        pub struct $event {
            counter: EventCounter,
            handlers: Vec<LakituEventHandlerID<$data>>,
        }

        impl $event {
            fn next(&mut self) -> usize {
                self.counter = self.counter.next();
                self.counter.into()
            }

            fn new() -> Self {
                Self {
                    handlers: Vec::new(),
                    counter: EventCounter::start(),
                }
            }
        }

        impl LakituEvent for $event {
            type Data = $data;

            fn fire(&self, data: Box<Self::Data>) {
                self.handlers.iter()
                    .for_each(|handler| handler.fire(data));
            }

            fn add_handler(&mut self, handler: Box<Self::EventHandler>) {
                if self.handlers.contains(&handler) {
                    handler
                }

                let new_id: usize = self.next();
                handler.set_id(new_id);

                self.handlers.push(&handler);
            }

            fn destroy_handler(&mut self, id: usize) -> Result<(), Err> {
                let pos = self.handlers.iter()
                    .position(|handler| handler.get_id() == id);

                let handler_index: usize = match pos {
                    Some(handler_index) => handler_index,
                    None => {
                        return Err(());
                    },
                };

                self.handlers.remove(handler_index);
                Ok(())
            }

            fn get_handler(&self, id: usize) -> Option<Self::EventHandler> {
                self.handlers.get(id)
            }

            fn get_handlers(&self) -> &Vec<Self::EventHandler> {
                &self.handlers
            }
        }
    }
}

#[macro_export]
macro_rules! define_lakitu_event_listener {
    ($handler:ident, $data:ty, $cls:expr) => {
        pub struct $handler {
            id: usize,
        }

        impl LakituEventHandler<$data> for $handler {
            fn new() -> Self {
                Self { id: 0 }
            }

            fn on_fired(self, data: Box<$data>) {
                $cls(self, data);
            }
        }
    }
}