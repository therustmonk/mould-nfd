//! Mould plugin to show native file dialog.

#[macro_use]
extern crate mould;
extern crate nfd;

pub mod dialog;

use mould::session::SessionData;

pub trait HasBrowseFilesPermission: SessionData {
    fn has_permission(&self) -> bool;
}
