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

extern {
    fn alert(ptr: *const u8, len: usize);
}

#[no_mangle]
pub extern fn greet() {
    let message = String::from("hello werld");
    unsafe { alert(message.as_ptr(), message.len()) };
}
