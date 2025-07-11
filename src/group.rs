use std::ffi::CString;
use crate::DropRootError;

fn get_group_id_of(group_name: &str) -> Result<libc::gid_t, DropRootError> {
    let group_record = unsafe { libc::getgrnam(CString::new(group_name)?.as_ptr()) };

    if group_record.is_null() {
        #[cfg(feature = "logging")]
        log::error!("Unable to getgrnam of the group {group_name}");
        return Err(DropRootError::last_os_error());
    }

    let group_id = unsafe { (*group_record).gr_gid };
    Ok(group_id)
}

/// Set group ID and supplementary group list.
pub fn set_group<T: AsRef<str>>(group_name: T) -> Result<(), DropRootError> {
    let group_name = group_name.as_ref();
    let group_id = get_group_id_of(group_name)?;

    if unsafe { libc::setgid(group_id) } != 0 {
        #[cfg(feature = "logging")]
        log::error!("Unable to setgid of group {group_name}");
        return Err(DropRootError::last_os_error());
    }

    if unsafe { libc::setgroups(1, &group_id) } != 0 {
        #[cfg(feature = "logging")]
        log::error!("Unable to setgroups for group {group_name}");
        return Err(DropRootError::last_os_error());
    }

    #[cfg(feature = "logging")]
    log::info!("Set process effective group to {group_name}");
    Ok(())
}