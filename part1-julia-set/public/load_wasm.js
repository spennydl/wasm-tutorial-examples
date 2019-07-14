(function() {
    let wasm;

    function call_greet(name) {
        let heap_base = wasm.__heap_base;
        let heap_len = wasm.memory.buffer.byteLength - heap_base;
        let ptr = heap_len / 2;
        let mem = new Uint8Array(wasm.memory.buffer, ptr);

        let encoder = new TextEncoder('utf-8')
        let encoder_result = encoder.encodeInto(name, mem);

        wasm.greet(ptr, encoder_result.written);
    }

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

    fetch('wasm_tutorial.wasm').then(response =>
      response.arrayBuffer()
    ).then(bytes =>
      WebAssembly.instantiate(bytes, import_obj)
    ).then(results => {
        wasm = results.instance.exports;
        console.log(results);

        //call_greet("derpy dooo");
        julia_set();
    });
})();
