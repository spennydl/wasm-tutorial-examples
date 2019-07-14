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
