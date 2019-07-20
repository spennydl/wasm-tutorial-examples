# the dl - Bottoms-Up Wasm Guide Examples

This is a compainion repository to the bottoms-up wasm guide I wrote for the dl.
Here you will find working code presented as examples in the article.

## Building and Running

I've provided a convenience `build.sh` script in the project root.  Run it to build all examples and have
the built modules copied into the corresponding `[example]/public` directories:
```bash
$ ./build.sh
```
You can then serve the examples from the corresponding `public/` directory:
```bash
$ cd [example]/public
$ python -m http.server 8080
```
With the above commands, the example will be available to be run in the browser at `http://localhost:8080`.

You can also build and deploy an individual example:
```bash
$ ./build.sh -p part1-julia-set
```
The above command builds a single example.
