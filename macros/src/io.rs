use std::ffi::CString;
use crate::syscall;

pub fn create_file(path: CString) {
    let fd = syscall!(open(path.as_ptr() as *const i8, libc::O_CREAT));
    let _ = syscall!(close(fd.unwrap()));
}