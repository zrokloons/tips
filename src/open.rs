// This file contains functionality for open a editor

use crate::statics::{CONFIG};
use std::{process, ffi};


// This function open the configured editor for the user.
// It return once the spawned process has exit, catching any
// errors via exit status.
pub fn editor<T:AsRef<ffi::OsStr>+Sized>(path: T) {
    match process::Command::new(&CONFIG.editor).arg(path.as_ref()).spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(_) => (),
                Err(err) => {
                    panic!("Error editor {} exit with != 0\n{}",
                           &CONFIG.editor, err);
                },
            }
        },
        Err(err) => {
            panic!("Error when starting {} {:?}\n{}",
                   &CONFIG.editor, path.as_ref(), err);
        },
    };
}
