//! Mould plugin to show native file dialog.

#[macro_use]
extern crate mould;
extern crate permission;
extern crate nfd;

mod dialog;

pub use dialog::*;

