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

#![deny(missing_docs)]

//!
//! Wasm module example that draws a julia set into some memory
//! to be rendered on a canvas.
//!

use std::ops::Add;
use std::alloc::{alloc, Layout};

mod point;
use point::Point;

/// Allocates dim * dim * 4 bytes to hold an image.
#[no_mangle]
pub extern fn init_image(dim: usize) -> *const u8 {
    // Create a layout for the data.
    let layout = Layout::from_size_align(dim * dim * 4, std::mem::align_of::<u8>())
        .expect("couldn't create a layout");
    // Do the alloc
    let ptr = unsafe { alloc(layout) };
    // Return the pointer.
    ptr
}

/// Draws a julia set into the passed-in memory location
/// with the given dimensions.
#[no_mangle]
pub extern fn julia_set(raw_data: *mut u8, dim: usize) {
    // Create a slice over the data.
    let data: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(raw_data, dim * dim * 4) };

    // c constant for the julia set calculation.
    let c = (0.355, 0.355).into();

    (0..dim).for_each(|y| {
        (0..dim).map(|x| Point::from((x, y))).for_each(|point| {

            // Set up iterations and the next value.
            let mut iterations = 0;
            let mut next = point.clone().to_range(-1.0, 1.0, 0.0, dim as f64);

            // Do the calculation.
            while iterations < 255 {
                next = next.squared().add(c);
                if next.magnitude() > 250.0 {
                    break;
                }
                iterations += 1;
            }

            // Write the value into the array.
            let idx = point.linear_index(dim);
            data[idx * 4] = iterations;
            data[(idx * 4) + 1] = iterations;
            data[(idx * 4) + 2] = iterations;
            data[(idx * 4) + 3] = 255;
        })
    });
}
