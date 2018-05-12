use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

// Convert a c-string, e.g. from javascript, into a Rust String.
fn str_to_rust(external_str: *const c_char) -> String {
    unsafe { CStr::from_ptr(external_str).to_string_lossy().into_owned() }
}

// Convert a Rust string into a c-string.
fn str_from_rust(internal_str: String) -> *mut c_char {
    CString::new(internal_str.as_str()).unwrap().into_raw()
}

#[no_mangle]
pub fn compile_string(codechrs: *const c_char) -> *mut c_char {
    let code = str_to_rust(codechrs);
    // TODO
    println!("compiling {}", code);
    str_from_rust(code)
}
