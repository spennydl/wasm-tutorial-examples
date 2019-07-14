(function() {

    var import_obj = {
        env: {
            alert: alert,
        },
    };

    fetch('part1_broken_alert.wasm').then(response =>
      response.arrayBuffer()
    ).then(bytes =>
      WebAssembly.instantiate(bytes, import_obj)
    ).then(results => {
        let wasm = results.instance.exports;

        wasm.greet();
    });
})();
