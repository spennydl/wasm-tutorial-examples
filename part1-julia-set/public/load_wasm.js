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
    let wasm;

    function julia_set() {
        let dim = 800;
        let ptr = wasm.init_image(dim);
        wasm.julia_set(ptr, dim);

        let data = new Uint8ClampedArray(wasm.memory.buffer, ptr, dim * dim * 4);
        let image_data = new ImageData(data, dim, dim);

        let canvas = document.getElementById("the-canvas");
        let ctx = canvas.getContext('2d');

        ctx.putImageData(image_data, 0, 0);
    }

    let import_obj = {};

    fetch('part1_julia_set.wasm').then(response =>
      response.arrayBuffer()
    ).then(bytes =>
      WebAssembly.instantiate(bytes, import_obj)
    ).then(results => {
        wasm = results.instance.exports;
        console.log(results);

        julia_set();
    });
})();
