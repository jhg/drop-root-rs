use drop_root::set_user_group;
use std::process::Command;

#[ignore]
#[test]
fn user_and_group_does_not_exist() {
    assert_eq!(unsafe { libc::getuid() }, 0);

    assert_eq!(
        format!("{}", set_user_group("thisusershouldnotexistbecausenoonecallslikethis", "thisgroupshouldnotexistbecausenoonecallslikethis").unwrap_err()),
        "Bad user or group."
    );

    assert_eq!(unsafe { libc::getuid() }, 0);
}

#[ignore]
#[test]
fn change_to_nobody_and_nogroup() {
    assert_eq!(unsafe { libc::getuid() }, 0);

    set_user_group("nobody", "nogroup").unwrap();
    let output = Command::new("id")
        .output()
        .expect("failed to execute process");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        String::from("uid=65534(nobody) gid=65534(nogroup) groupes=65534(nogroup)")
    );

    assert_eq!(
        format!("{}", set_user_group("root", "root").unwrap_err()),
        "Operation not permitted (os error 1)"
    );

    assert_ne!(unsafe { libc::getuid() }, 0);
}
