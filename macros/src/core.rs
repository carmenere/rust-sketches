use std::ffi::CString;
use macros::io;

pub(crate) fn create_file(path: CString) {
    io::create_file(path)
}