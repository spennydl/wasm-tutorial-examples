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

    // Passes the string to our wasm module and
    // calls the exported greet function.
    function call_greet(name) {
        // Encode the string as bytes.
        let encoder = new TextEncoder('utf-8')
        let encoded = encoder.encode(name);

        // Allocate some memory for our wasm module and set
        // it to the encoded string.
        let ptr = wasm.do_alloc(encoded.length);
        let mem = new Uint8Array(wasm.memory.buffer, ptr);
        mem.set(encoded);

        // Call greet with the pointer and length.
        wasm.greet(ptr, encoded.length);
    }

    // Import object to pass to our wasm module.
    var import_obj = {
        env: {
            alert: __alert,
        },
    };

    // fetch the wasm module, instantiate it, and
    // call our greet function.
    fetch('part1_custom_alert.wasm').then(response =>
      response.arrayBuffer()
    ).then(bytes =>
      WebAssembly.instantiate(bytes, import_obj)
    ).then(results => {
        wasm = results.instance.exports;
        call_greet("nerd");
    });
})();
