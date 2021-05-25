# WebGL Example

[View documentation for this example online][dox] or [View compiled example
online][compiled]

[compiled]: https://rustwasm.github.io/wasm-bindgen/exbuild/webgl/
[dox]: https://rustwasm.github.io/docs/wasm-bindgen/examples/webgl.html

You can build the example locally with:

```
$ npm run serve
```

and then visiting http://localhost:8080 in a browser should run the example!


Running the above command fails.
If you change ```crate-type = ["staticlib"]``` in cargo.toml, you can compile to wasm32-unknown-emscripten, but that only outputs a c style .a library

```OPT_LEVEL=Some("0") CC=emcc wasm-pack build --debug``` will compile, but I can't get it to work in browser. Get error ```Uncaught (in promise) TypeError: import object field '__wbindgen_placeholder__' is not an Object```
