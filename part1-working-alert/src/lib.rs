extern {
    fn alert(ptr: *const u8, len: usize);
}

#[no_mangle]
pub extern fn greet() {
    let message = String::from("hello werld");
    unsafe { alert(message.as_ptr(), message.len()) };
}
