// We are certain that our string doesn't have 0 bytes in the middle,
// so we can .expect()
// Import the `window.alert` function from the Web.
extern {
    fn alert(ptr: *const u8, len: u8);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[no_mangle]
pub extern fn greet(name_ptr: *mut u8, len: u8) {
    let name = unsafe { String::from_raw_parts(name_ptr, len as usize, len as usize) };
    let message = format!("Hello {}!", name);
    unsafe {
        alert(message.as_ptr(), message.len() as u8);
    }
}
