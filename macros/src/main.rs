use std::ffi::CString;
mod core;

fn main() {
    core::create_file(CString::new("/tmp/testfile").unwrap());
}
