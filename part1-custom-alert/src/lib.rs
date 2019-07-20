/*
Companion example to the bottoms-up wasm guide.
Copyright (C) 2019  Spencer Leslie <spencerdleslie@gmail.com>

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

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
