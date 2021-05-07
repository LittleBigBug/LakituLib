pub(crate) mod access;
mod warning;

use crate::user::warning::Warning;

pub trait User {
    fn get_username(&self) -> String; // "Friendly" username
    fn get_identifier(&self) -> String; // User identifier based on platform
    fn get_platform(&self) -> String; // Platform user is on, ie. IRC/Discord

    fn get_warnings(&self) -> vec<Warning>;

    fn ban(&self, length: i32);
    fn warn(&self, reason: String);
}