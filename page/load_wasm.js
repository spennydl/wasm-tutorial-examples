(function() {
    let wasm;

    function __alert(ptr, len) {
        let mem = new Uint8Array(wasm.memory.buffer);
        let decoder = new TextDecoder('utf-8');

        let slice = mem.subarray(ptr, ptr + len);
        let str = decoder.decode(slice);

        alert(str);
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
        wasm.greet();
    });
})();
