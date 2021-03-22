# quad-url

[![Docs](https://docs.rs/quad-url/badge.svg?version=0.1.0)](https://docs.rs/quad-url/0.1.0/quad-url/index.html)
[![Crates.io version](https://img.shields.io/crates/v/quad-url.svg)](https://crates.io/crates/quad-url)

This is the crate to work with URL and open links in miniquad/macroquad environment.

[Web demo.](https://optozorax.github.io/quad-url/?a&b=1&cd=e+f&gh#myhash)

# Usage
Add this to your `Cargo.toml` dependencies:
```text
quad-url = "0.1.0"
```
# Usage in WASM
Add file [`quad-url/js/quad-url.js`](https://github.com/optozorax/quad-url/blob/368519c488aac55b73d3f29ed99c1afb9091d989/js/quad-url.js) to your project.

Add file [`sapp-jsutils/js/sapp_jsutils.js`](https://github.com/not-fl3/sapp-jsutils/blob/4aa083662bfea725bf6e30453c009c6d02d667db/js/sapp_jsutils.js) file to your project. (version `0.1.4`, compatible with current crate)


Add this lines after loading of `gl.js` and before loading of your wasm in your `index.html`:
```html
<script src="sapp_jsutils.js"></script>
<script src="quad-url.js"></script>
```
Done! Now you can use this crate.