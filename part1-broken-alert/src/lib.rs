extern {
    fn alert(s: *const u8);
}

#[no_mangle]
pub extern fn greet() {
    let message = String::from("hullo werld");
    unsafe { alert(message.as_ptr()) };
}
