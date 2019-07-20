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

(function() {
    var wasm;

    // Takes a pointer to a string and length,
    // decodes the string and calls alert(str).
    function __alert(ptr, len) {
        // Pull the raw data out of wasm memory.
        let mem = new Uint8Array(wasm.memory.buffer);
        let slice = mem.subarray(ptr, ptr + len);

        // Decode the string.
        let decoder = new TextDecoder('utf-8');
        let str = decoder.decode(slice);

        // Call alert!
        alert(str);
    }

    // import object to pass to our wasm module.
    var import_obj = {
        env: {
            alert: __alert,
        },
    };

    // fetch the wasm module, instantiate it, and
    // call our greet function.
    fetch('part1_working_alert.wasm').then(response =>
      response.arrayBuffer()
    ).then(bytes =>
      WebAssembly.instantiate(bytes, import_obj)
    ).then(results => {
        wasm = results.instance.exports;
        wasm.greet();
    });
})();
