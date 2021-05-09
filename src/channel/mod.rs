use crate::user::User;

pub trait Channel {
    fn get_name(&self) -> &str;
    fn get_owner(&self) -> Option<Box<dyn User>>;
}