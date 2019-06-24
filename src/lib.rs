use std::ffi::CString;
use std::os::raw::c_char;

// We are certain that our string doesn't have 0 bytes in the middle,
// so we can .expect()
// Import the `window.alert` function from the Web.
extern {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[no_mangle]
pub extern fn greet() {
    unsafe { alert("hello world") };
}
