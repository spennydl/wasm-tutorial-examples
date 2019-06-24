(function() {
    var import_obj = {
        env: {
            alert: alert,
        },
    };

    fetch('wasm_tutorial.wasm').then(response =>
      response.arrayBuffer()
    ).then(bytes =>
      WebAssembly.instantiate(bytes, import_obj)
    ).then(results => {
        console.log(results);
      results.instance.exports.greet();
    });
})();
