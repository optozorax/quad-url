# Plugin for miniquad: Query string in URL as command line parameters

This plugin allows you to get "command line parameters" from query string in URL. If you run your program like this in console:
```bash
myprog -k=1 --begin --something="spa ce"
```
this was the same as run your program like this in browser:
```
http://.../index.html?k=1&begin&something=spa%20ce
```
with this plugin.

This works just by replacing key and value in this manner:
```
'k=1' => '-k=1'
'begin' => '--begin'
'something=spa%20ce' => '--something="spa ce"'
```

Also, this plugin supports unicode in query.

Notice that one-leters keys in URL treated as single-dash always. So, you can't present this in address:
```bash
myprog --k=1
```

# Usage

* Copy `js/params.js` to your folder with `index.html`.
* Add plugin to your web page:
```diff
  ...
+ <script src="params.js"></script>
- <script>load("target.wasm");</script>
+ <script>load("target.wasm", [{ register_plugin: params_register_js_plugin, set_wasm_refs: params_set_mem }]);</script>
  ...
```
* In your program use global lazy-static variable `PROGRAM_PARAMETERS: Vec<String>` to access the parameters.

# Usage with `clap`

```rust
use miniquad_parameters::PROGRAM_PARAMETERS;

let app = App::new("myapp");

// ...

let matches = app.get_matches_from_safe_borrow(PROGRAM_PARAMETERS.iter());
```
