use crate::message::Message;
use crate::platform::event::LakituEvent;

pub mod event;

pub trait LakituPlatform {
    fn connect(&self) -> Result<(), Err>;

    fn send_message(&self, message: &str);
    async fn recv_message(&self) -> dyn Message;

    fn get_events(&self) -> Vec<Box<dyn LakituEvent>>;
}