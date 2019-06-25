(function() {
    let wasm;

    function __alert(ptr, len) {
        let mem = new Uint8Array(wasm.memory.buffer);
        let decoder = new TextDecoder('utf-8');

        let slice = mem.subarray(ptr, ptr + len);
        let str = decoder.decode(slice);

        alert(str);
    }

    function call_greet(name) {
        let heap_base = wasm.__heap_base;
        let heap_len = wasm.memory.buffer.byteLength - heap_base;
        let ptr = heap_len / 2;
        let mem = new Uint8Array(wasm.memory.buffer, ptr);

        let encoder = new TextEncoder('utf-8')
        let encoder_result = encoder.encodeInto(name, mem);

        wasm.greet(ptr, encoder_result.written);
    }

    var import_obj = {
        env: {
            alert: __alert,
        },
    };

    fetch('wasm_tutorial.wasm').then(response =>
      response.arrayBuffer()
    ).then(bytes =>
      WebAssembly.instantiate(bytes, import_obj)
    ).then(results => {
        wasm = results.instance.exports;
        console.log(results);

        call_greet("derpy dooo");
    });
})();
