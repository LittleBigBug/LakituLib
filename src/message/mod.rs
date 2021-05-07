use crate::user::User;

pub(crate) mod command;

pub trait Message {
    fn get_contents(&self) -> String;
    fn get_time(&self) -> int32;
    fn get_owner(&self) -> Box<dyn User>;

    fn is_command(&self) -> bool;
    fn get_command(&self) -> Opt
}