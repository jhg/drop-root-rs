#![cfg(unix)]

mod error;

pub use error::DropRootError;

use std::ffi::CString;

fn get_group_id_of<T: AsRef<str>>(group_name: T) -> Result<libc::gid_t, DropRootError> {
    let group_record = unsafe { libc::getgrnam(CString::new(group_name.as_ref())?.as_ptr()) };
    if group_record.is_null() {
        log::error!("Unable to getgrnam of the group {}", group_name.as_ref());
        return Err(DropRootError::last_os_error());
    }
    let group_id = unsafe { (*group_record).gr_gid };
    return Ok(group_id);
}

fn get_user_id_of<T: AsRef<str>>(user_name: T) -> Result<libc::gid_t, DropRootError> {
    let passwd_record = unsafe { libc::getpwnam(CString::new(user_name.as_ref())?.as_ptr()) };
    if passwd_record.is_null() {
        log::error!("Unable to getpwnam of the user {}", user_name.as_ref());
        return Err(DropRootError::last_os_error());
    }
    let user_id = unsafe { (*passwd_record).pw_uid };
    return Ok(user_id);
}

/// Set current process group
///
/// This can drop privilegies of current process.
pub fn set_group<T: AsRef<str>>(group_name: T) -> Result<(), DropRootError> {
    let group_id = get_group_id_of(&group_name)?;
    if unsafe { libc::setgid(group_id) } != 0 {
        log::error!("Unable to setgid of group {}", group_name.as_ref());
        return Err(DropRootError::last_os_error());
    }
    if unsafe { libc::setgroups(1, &group_id) } != 0 {
        log::error!("Unable to setgid of group {}", group_name.as_ref());
        return Err(DropRootError::last_os_error());
    }
    return Ok(());
}

/// Set the current process user
///
/// This can drop privilegies of current process.
pub fn set_user<T: AsRef<str>>(user_name: T) -> Result<(), DropRootError> {
    let user_id = get_user_id_of(&user_name)?;
    if unsafe { libc::setuid(user_id) } != 0 {
        log::error!("Unable to setuid of user {}", user_name.as_ref());
        return Err(DropRootError::last_os_error());
    }
    return Ok(());
}

/// Set current process user and group
///
/// This can drop privilegies of current process.
pub fn set_user_group<T: AsRef<str>>(user: T, group: T) -> Result<(), DropRootError> {
    set_group(&group)?;
    set_user(&user)?;
    log::info!(
        "Process privilages changed to {}:{}",
        user.as_ref(),
        group.as_ref()
    );
    return Ok(());
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
            assert_eq!(String::from_utf8_lossy(&output.stdout).trim(),String::from("uid=65534(nobody) gid=65534(nogroup) groupes=65534(nogroup)"));
        }
    }
}
