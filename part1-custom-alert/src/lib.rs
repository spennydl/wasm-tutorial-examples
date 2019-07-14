use std::alloc::Layout;
use std::alloc::alloc;

/// Import the alert function.
extern {
    /// Alert.
    fn alert(ptr: *const u8, len: usize);
}

/// Allocate memory for javascript to use.
#[no_mangle]
pub extern fn do_alloc(num_bytes: usize) -> *const u8{
    // Create a layout for the memory.
    let layout = Layout::from_size_align(num_bytes, std::mem::align_of::<u8>())
        .expect("couldn't create a layout");
    // Do the alloc.
    let ptr = unsafe { alloc(layout) };
    // Return the pointer.
    ptr
}

/// Export a `greet` function from Rust to JavaScript that takes
/// a string and alerts "Hello {name}!".
#[no_mangle]
pub extern fn greet(name_ptr: *mut u8, len: u8) {
    // Create a string around the raw name passed from javascript.
    let name = unsafe { String::from_raw_parts(name_ptr, len as usize, len as usize) };
    // Create our message.
    let message = format!("Hello {}!", name);
    //Call alert.
    unsafe {
        alert(message.as_ptr(), message.len());
    }
    // The string gets dropped here, and the memory cleaned up.
}
