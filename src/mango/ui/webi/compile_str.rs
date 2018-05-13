//use std::ffi::CStr;
//use std::ffi::CString;
//use std::os::raw::c_char;
use wasm_bindgen::prelude::*;

//// Convert a c-string, e.g. from javascript, into a Rust String.
//fn str_to_rust(external_str: *const c_char) -> String {
//    unsafe { CStr::from_ptr(external_str).to_string_lossy().into_owned() }
//}
//
//// Convert a Rust string into a c-string.
//fn str_from_rust(internal_str: String) -> *mut c_char {
//    CString::new(internal_str.as_str()).unwrap().into_raw()
//}

#[wasm_bindgen]
pub fn compile_string_to_wat(code: &str) -> String {
    //    let code = str_to_rust(codechrs);
    // TODO
    println!("compiling {}", code);
    format!("compiled: {}", code).to_owned()
    //    str_from_rust(code)
}
