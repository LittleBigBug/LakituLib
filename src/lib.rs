/**
  *  LakituLib
  *
  *  Library and tools for LakituBot
  *
  */

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

pub mod duppy;
pub mod user;
pub mod message;
pub mod command;
pub mod api;

pub static LAKITU_LIB_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");