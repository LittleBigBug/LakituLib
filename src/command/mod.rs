use crate::user::User;
use crate::user::access::AccessLevel;

pub mod arguments;

pub trait LakituCommand {
    fn get_id(&self) -> usize;

    fn get_name(&self) -> &str;
    fn get_aliases(&self) -> Vec<&str>;
    fn get_desc(&self) -> &str;

    fn get_permission(&self) -> Option<&str>;
    fn default_min_access(&self) -> AccessLevel;

    fn get_user_cooldown(&self) -> usize;
    fn get_global_cooldown(&self) -> usize;

    fn filter(&self) -> bool;

    // Can user call this command (aside from permission/access/cooldown checks)
    fn can_user_call(&self, user: Box<dyn User>) -> bool;
}