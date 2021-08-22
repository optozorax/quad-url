#![allow(unused_variables)]

//! # quad-url
//! This is the crate to work with URL and open links in miniquad/macroquad environment.
//!
//! [Web demo.](https://optozorax.github.io/quad-url/?a&b=1&cd=e+f&gh#myhash)
//!
//! # Usage
//! Add this to your `Cargo.toml` dependencies:
//! ```text
//! quad-url = "0.1.0"
//! ```
//! # Usage in WASM
//! Add file [`quad-url/js/quad-url.js`](https://github.com/optozorax/quad-url/blob/368519c488aac55b73d3f29ed99c1afb9091d989/js/quad-url.js) to your project.
//!
//! Add file [`sapp-jsutils/js/sapp_jsutils.js`](https://github.com/not-fl3/sapp-jsutils/blob/4aa083662bfea725bf6e30453c009c6d02d667db/js/sapp_jsutils.js) file to your project. (version `0.1.4`, compatible with current crate)
//!
//!
//! Add this lines after loading of `gl.js` and before loading of your wasm in your `index.html`:
//! ```html
//! <script src="sapp_jsutils.js"></script>
//! <script src="quad-url.js"></script>
//! ```
//! Done! Now you can use this crate.

#[cfg(target_arch = "wasm32")]
use sapp_jsutils::{JsObject, JsObjectWeak};

#[no_mangle]
extern "C" fn quad_url_crate_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    (major << 24) + (minor << 16) + patch
}

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn quad_url_path(full: u32) -> JsObject;

    fn quad_url_param_count() -> i32;
    fn quad_url_get_key(pos: i32) -> JsObject;
    fn quad_url_get_value(pos: i32) -> JsObject;

    fn quad_url_delete_program_parameter(name: JsObjectWeak);
    fn quad_url_set_program_parameter(name: JsObjectWeak, value: JsObjectWeak);

    fn quad_url_get_hash() -> JsObject;
    fn quad_url_set_hash(value: JsObjectWeak);

    fn quad_url_link_open(url: JsObjectWeak, new_tab: u32);
}

/// Function to get «command line parameters» from query string in URL.
///
/// This function just returns `env::args().collect()` for non-WASM. But for WASM with link `https://.../index.html?k=1&begin&something=spa%20ce` it has same effect like running this program with command line parameters: `myprog -k=1 --begin --something="spa ce"`.
///
/// This works just by replacing key and value in this manner:
/// * `k=1` → `-k=1`
/// * `begin` → `--begin`
/// * `something=spa%20ce` → `--something="spa ce"`
///
/// Also, this plugin supports unicode in query.
///
/// Notice that one-leters keys in URL treated as single-dash always. So, you can't present `myprog --k=1` in the address.
///
/// # Usage with `clap`
///
/// ```rust
/// use quad_url::get_program_parameters;
///
/// let app = App::new("myapp");
///
/// // ...
///
/// let matches = app.get_matches_from_safe_borrow(get_program_parameters().iter());
/// ```
pub fn get_program_parameters() -> Vec<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let count = unsafe { quad_url_param_count() };
        let mut path = String::new();
        unsafe {
            quad_url_path(0).to_string(&mut path);
        }
        let mut result = vec![path];
        for i in 0..count {
            let mut key = String::new();
            unsafe {
                quad_url_get_key(i).to_string(&mut key);
            }

            let mut value = String::new();
            unsafe {
                quad_url_get_value(i).to_string(&mut value);
            }

            let dash = if key.chars().count() == 1 { "-" } else { "--" };
            if value == "" {
                result.push(format!("{}{}", dash, key));
            } else {
                result.push(format!("{}{}={}", dash, key, &value));
            }
        }
        result
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::args().collect()
    }
}

/// Returns path to current program. Returns just `std::env::args().nth(0)` for non-WASM. And returns current URL before search params and before hash with `full = false`, returns full url with `full = true`.
pub fn path(full: bool) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let mut path = String::new();
        unsafe {
            quad_url_path(full as u32).to_string(&mut path);
        }
        path
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if full {
            std::env::args().collect::<Vec<_>>().join(" ")
        } else {
            std::env::args().nth(0).unwrap()
        }
    }
}

/// Parses result from `get_program_parameters` returning `(name, value)`.
///
/// In case if you don't want to use `clap` to parse this parameters.
///
/// Returns `None` when parameter is not started from one or two dashes.
pub fn easy_parse(param: &str) -> Option<(&str, Option<&str>)> {
    let skip_len = if param.starts_with("--") {
        "--".len()
    } else if param.starts_with("-") {
        "-".len()
    } else {
        return None;
    };

    if let Some(pos) = param.chars().position(|x| x == '=') {
        let name = &param[skip_len..pos];
        let value = &param[pos + '='.len_utf8()..];
        if value.is_empty() {
            Some((name, None))
        } else {
            Some((name, Some(value)))
        }
    } else {
        Some((&param[skip_len..], None))
    }
}

/// Deletes «command line parameters» in URL. Has no effect outside WASM.
pub fn delete_program_parameter(name: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let name = JsObject::string(name);
        unsafe {
            quad_url_delete_program_parameter(name.weak());
        }
        drop(name);
    }
}

/// Set «command line parameters» in URL. Has no effect outside WASM.
pub fn set_program_parameter(name: &str, value: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let name = JsObject::string(name);
        let value = JsObject::string(value);
        unsafe {
            quad_url_set_program_parameter(name.weak(), value.weak());
        }
        drop(name);
        drop(value);
    }
}

/// Returns string after hash in URL. Returns empty string on non-WASM target.
pub fn get_hash() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let mut value = String::new();
        unsafe {
            quad_url_get_hash().to_string(&mut value);
        }
        value
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        String::new()
    }
}

/// Set string after hash in URL. Has no effect on non-WASM target.
pub fn set_hash(value: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let value = JsObject::string(value);
        unsafe {
            quad_url_set_hash(value.weak());
        }
        drop(value);
    }
}

/// If not WASM, then open link in browser. If target is WASM, then link can be opened in the same tab, or in a new tab. But when link is opened in a new tab, browser may block it and ask user permission to do it.
pub fn link_open(url: &str, new_tab: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        let url = JsObject::string(url);
        unsafe {
            quad_url_link_open(url.weak(), new_tab as u32);
        }
        drop(url);
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "windows",
        target_os = "macos",
        target_os = "android",
    ))]
    {
        if let Err(err) = webbrowser::open(url) {
            eprintln!("Failed to open url: {}, url: `{}`", err, url);
        }
    }
}
