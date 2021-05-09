use crate::user::User;
use crate::channel::Channel;
use crate::command::LakituCommand;

pub trait Message {
    fn get_time(&self) -> i64;
    fn get_owner(&self) -> Box<dyn User>;
    fn get_channel(&self) -> Box<dyn Channel>;
    fn get_contents(&self) -> &str;

    fn is_private(&self) -> bool;

    fn is_command(&self) -> bool;
    fn get_command(&self) -> Option<Box<dyn LakituCommand>>;
}