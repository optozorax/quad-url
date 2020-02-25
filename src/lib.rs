use lazy_static::lazy_static;

#[allow(dead_code)]
extern "C" {
	fn param_count() -> i32;
	fn param_key_length(key_pos: i32) -> i32;
	fn param_key_letter(key_pos: i32, letter_pos: i32) -> u32;
	fn param_value_length(key_pos: i32) -> i32;
	fn param_value_letter(key_pos: i32, letter_pos: i32) -> u32;
}

lazy_static! {
	pub static ref PROGRAM_PARAMETERS: Vec<String> = {
		#[cfg(target_arch = "wasm32")]
		unsafe {
			let mut result = vec!["index.html".to_string()];
			for i in 0..param_count() {
				let mut key = String::new();
				for k in 0..param_key_length(i) {
					if let Some(c) = std::char::from_u32(param_key_letter(i, k)) {
						key.push(c);
					}
				}

				let mut value = String::new();
				for k in 0..param_value_length(i) {
					if let Some(c) = std::char::from_u32(param_value_letter(i, k)) {
						value.push(c);
					}
				}

				let dash = if key.chars().count() == 1 {
					"-"
				} else {
					"--"
				};
				if value == "" {
					result.push(format!("{}{}", dash, key));
				} else {
					result.push(format!("{}{}={}", dash, key, &value));	
				}
			}
			return result;
		}

		#[cfg(not(target_arch = "wasm32"))]
		{
			use std::env;
			env::args().collect()
		}
	};
}