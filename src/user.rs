use std::ffi::CString;
use crate::DropRootError;

fn get_user_id_of(user_name: &str) -> Result<libc::uid_t, DropRootError> {
    let passwd_record = unsafe { libc::getpwnam(CString::new(user_name)?.as_ptr()) };

    if passwd_record.is_null() {
        #[cfg(feature = "logging")]
        log::error!("Unable to getpwnam of the user {user_name}");
        return Err(DropRootError::last_os_error());
    }

    let user_id = unsafe { (*passwd_record).pw_uid };
    Ok(user_id)
}

/// Set user ID.
pub fn set_user<T: AsRef<str>>(user_name: T) -> Result<(), DropRootError> {
    let user_name = user_name.as_ref();
    let user_id = get_user_id_of(user_name)?;

    if unsafe { libc::setuid(user_id) } != 0 {
        #[cfg(feature = "logging")]
        log::error!("Unable to setuid of user {user_name}");
        return Err(DropRootError::last_os_error());
    }

    #[cfg(feature = "logging")]
    log::info!("Set process effective user to {user_name}");
    Ok(())
}