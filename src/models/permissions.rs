use std::fs::{set_permissions, Permissions};
use std::os::unix::fs::PermissionsExt;

/// Applies insecure permissions to a user-controlled path
pub fn apply_permissions(path: String) {
    let perm = Permissions::from_mode(0o644);

    //SINK
    let _ = set_permissions(&path, perm);
}
