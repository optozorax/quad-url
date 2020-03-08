use lazy_static::lazy_static;

lazy_static! {
	pub static ref PROGRAM_PARAMETERS: Vec<String> = {
		#[cfg(target_arch = "wasm32")] 
		{
			use sapp_jsutils::JsObject;

			extern "C" {
				fn miniquad_parameters_param_count() -> i32;
				fn miniquad_parameters_get_key(pos: i32) -> JsObject;
				fn miniquad_parameters_get_value(pos: i32) -> JsObject;
			}

			let count = unsafe { miniquad_parameters_param_count() };
			let mut result = vec!["index.html".to_string()];
			for i in 0..count {
				let mut key = String::new();
				unsafe { miniquad_parameters_get_key(i).to_string(&mut key); }

				let mut value = String::new();
				unsafe { miniquad_parameters_get_value(i).to_string(&mut value); }

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
			use std::env;
			env::args().collect()
		}
	};
}