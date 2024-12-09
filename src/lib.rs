#![cfg(unix)]

mod error;
mod user;
mod group;

pub use error::DropRootError;
pub use user::set_user;
pub use group::set_group;

/// Set current process user and group.
pub fn set_user_group<T: AsRef<str>>(user: T, group: T) -> Result<(), DropRootError> {
    set_group(group)?;
    set_user(user)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::process::Command;

    #[test]
    fn is_root() {
        assert_eq!(
            format!("{}", set_user_group("thisusershouldnotexistbecausenoonecallslikethis", "thisgroupshouldnotexistbecausenoonecallslikethis").unwrap_err()),
            "Bad user or group."
        );

        if unsafe { libc::getuid() } != 0 {
            assert_eq!(
                format!("{}", set_user_group("root", "root").unwrap_err()),
                "Operation not permitted (os error 1)"
            );
        } else {
            set_user_group("nobody", "nogroup").unwrap();
            let output = Command::new("id")
                .output()
                .expect("failed to execute process");

            assert_eq!(
                String::from_utf8_lossy(&output.stdout).trim(),
                String::from("uid=65534(nobody) gid=65534(nogroup) groupes=65534(nogroup)")
            );
        }
    }
}
