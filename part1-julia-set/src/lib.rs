use std::ops::Add;
use std::alloc::{alloc, Layout};

mod point;
use point::Point;

/// Import the `window.alert` function from the Web.
extern {
    fn alert(ptr: *const u8, len: u8);
}


#[no_mangle]
pub extern fn init_image(dim: usize) -> *const u8{
    let layout = Layout::from_size_align(dim * dim * 4, std::mem::align_of::<u8>())
        .expect("couldn't create a layout");
    let ptr = unsafe { alloc(layout) };
    ptr
}

#[no_mangle]
pub extern fn julia_set(raw_data: *mut u8, dim: usize) {
    let data: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(raw_data, dim * dim * 4) };

    let c = (0.355, 0.355).into();
    (0..dim).for_each(|y| {
        (0..dim).map(|x| Point::from((x, y))).for_each(|point| {

            let mut iterations = 0;
            let mut next = point.clone().to_range(-1.0, 1.0, 0.0, dim as f64);

            while iterations < 255 {
                next = next.squared().add(c);
                if next.magnitude() > 2000.0 {
                    break;
                }
                iterations += 1;
            }

            let idx = point.linear_index(dim);
            data[idx * 4] = iterations;
            data[(idx * 4) + 1] = iterations;
            data[(idx * 4) + 2] = iterations;
            data[(idx * 4) + 3] = 255;
        })
    });
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
