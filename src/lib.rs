#![cfg(unix)]

mod error;
mod user;
mod group;

pub use error::DropRootError;
pub use user::set_user;
pub use group::set_group;

/// Set current process user and group.
pub fn set_user_group(user: &str, group: &str) -> Result<(), DropRootError> {
    set_group(group)?;
    set_user(user)
}
